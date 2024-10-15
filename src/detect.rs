use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::CrateVersion;

#[derive(clap::Args)]
#[command(about, long_about = None)]
pub struct DetectMissingTags {
    #[arg(long)]
    crate_name: String,
    #[arg(long)]
    changelog_path: String,
    #[arg(long, default_value = "")]
    tag_prefix: String,
}

impl DetectMissingTags {
    pub fn run(&self) -> anyhow::Result<Vec<VersionToTag>> {
        let crate_versions: HashMap<String, CrateVersion> =
            CrateVersion::for_crate(&self.crate_name)?
                .into_iter()
                .map(|crate_version| (crate_version.name.clone(), crate_version))
                .collect();
        // println!("{:?}", crate_versions);

        let mut versions_to_tag = Vec::new();
        for release in parse_changelog::parse_iter(&std::fs::read_to_string(&self.changelog_path)?)
        {
            if release.version == "Unreleased" {
                continue;
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
                tagname: format!("{}{}", self.tag_prefix, release.version),
                commit_hash,
                title: release.title.to_owned(),
                notes: release.notes.to_owned(),
            });
        }
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
    pub title: String,
    pub notes: String,
}
