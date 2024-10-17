use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use crates_io_api::SyncClient;
use flate2::read::GzDecoder;
use indicatif::ProgressStyle;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::{ffi::OsString, io::Read};
use tar::Archive;
use tracing_indicatif::span_ext::IndicatifSpanExt;

/// Generate YAML with versions that don't have GitHub releases.
#[derive(clap::Args)]
pub struct DetectMissingTags {
    #[arg(long)]
    crate_name: String,
    #[arg(long)]
    changelog_path: String,
    #[arg(long)]
    tag_prefix: String,
    #[arg(long)]
    title_prefix: Option<String>,
    #[arg(long, action)]
    /// Generate release entries in the YAML even for versions that already have a tag.
    include_existing: bool,
}

impl DetectMissingTags {
    pub fn run(&self) -> anyhow::Result<Vec<VersionToTag>> {
        let existing_tags_to_skip = if !self.include_existing {
            retrieve_all_tags()?
        } else {
            Default::default()
        };

        let mut releases_to_check = Vec::new();

        let changelog_text = std::fs::read_to_string(&self.changelog_path)?;
        for release in parse_changelog::parse_iter(&changelog_text) {
            if release.version == "Unreleased" {
                continue;
            }
            let tagname = format!("{}{}", self.tag_prefix, release.version);
            if existing_tags_to_skip.contains(&tagname) {
                continue;
            }
            releases_to_check.push((release, tagname));
        }

        if releases_to_check.is_empty() {
            return Ok(Vec::default());
        }

        let mut versions_to_tag = Vec::new();
        let span = tracing::warn_span!("Fetching"); // use warn to show it by default
        span.pb_set_style(&ProgressStyle::default_bar());
        span.pb_set_length(releases_to_check.len() as u64 + 1);
        let span_enter = span.enter();

        let crate_versions: HashMap<String, CrateVersion> =
            CrateVersion::for_crate(&self.crate_name)?
                .into_iter()
                .map(|crate_version| (crate_version.name.clone(), crate_version))
                .collect();
        span.pb_inc(1);

        for (release, tagname) in releases_to_check {
            scopeguard::defer! {
                span.pb_inc(1);
            }

            let Some(crate_version) = crate_versions.get(release.version) else {
                tracing::warn!(
                    version = release.version,
                    "Cannot find a crates.io release for changelog entry"
                );
                continue;
            };
            let Some(commit_hash) = crate_version.resolve_commit_hash()? else {
                tracing::warn!(
                    version = release.version,
                    "Cannot resolve commit hash for crates.io release"
                );
                continue;
            };
            versions_to_tag.push(VersionToTag {
                version: release.version.to_owned(),
                tagname,
                commit_hash,
                created_at: crate_version.created_at,
                title: if let Some(title_prefix) = self.title_prefix.as_ref() {
                    format!("{} {}", title_prefix, release.title)
                } else {
                    release.title.to_owned()
                },
                notes: release.notes.to_owned(),
            });
        }
        drop(span_enter);
        versions_to_tag
            .sort_by_key(|version| Some(crate_versions.get(&version.version)?.created_at));
        Ok(versions_to_tag)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionToTag {
    pub version: String,
    pub tagname: String,
    pub commit_hash: String,
    pub created_at: DateTime<Utc>,
    pub title: String,
    pub notes: String,
}

fn retrieve_all_tags() -> anyhow::Result<HashSet<String>> {
    let repo = gix::open(".")?;
    let mut tag_names = HashSet::new();
    for tag in repo.references()?.tags()? {
        tag_names.insert(tag.unwrap().name().file_name().to_string());
    }
    Ok(tag_names)
}

#[derive(Debug)]
pub struct CrateVersion {
    pub name: String,
    pub created_at: DateTime<Utc>,
    dl_url: Url,
}

impl CrateVersion {
    pub fn for_crate(crate_name: &str) -> anyhow::Result<Vec<CrateVersion>> {
        let client = SyncClient::new(
            "bevy-tnua-physics-integration-layer (https://github.com/idanarye/retrospective-crate-version-tagging)",
            std::time::Duration::from_millis(1000),
        )?;
        tracing::info!(crate = crate_name, "Getting crate information");
        let crate_data = client.get_crate(crate_name)?;
        let base_url =
            Url::parse("https://crates.io/").expect("Statically defined URL should work");
        crate_data
            .versions
            .into_iter()
            .map(|version_data| {
                Ok(CrateVersion {
                    name: version_data.num,
                    created_at: version_data.created_at,
                    dl_url: base_url.join(&version_data.dl_path)?,
                })
            })
            .collect()
    }

    pub fn resolve_commit_hash(&self) -> anyhow::Result<Option<String>> {
        tracing::info!(version = self.name, "Resolving commit hash for version");
        Ok(extract_commit_hash(reqwest::blocking::get(
            self.dl_url.clone(),
        )?)?)
    }
}

pub fn extract_commit_hash(input: impl Read) -> Result<Option<String>, std::io::Error> {
    let decoder = GzDecoder::new(input);
    let mut a = Archive::new(decoder);

    let desired_filename = OsString::from(".cargo_vcs_info.json");

    for file in a.entries()? {
        let entry = file?;
        let path = entry.header().path()?;
        if path.file_name() != Some(&desired_filename) {
            continue;
        }
        // TODO: Use a proper struct instead of serde_json::Value
        let value = serde_json::from_reader::<_, serde_json::Value>(entry)?;
        return Ok(value["git"]["sha1"].as_str().map(|s| s.to_owned()));
    }
    Ok(None)
}
