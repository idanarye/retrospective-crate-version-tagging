use std::collections::{HashMap, HashSet};

use chrono::{DateTime, Utc};
use indicatif::ProgressStyle;
use serde::{Deserialize, Serialize};
use tracing_indicatif::span_ext::IndicatifSpanExt;

use crate::CrateVersion;

#[derive(clap::Args)]
#[command(about, long_about = None)]
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
