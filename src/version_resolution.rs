use chrono::{DateTime, Utc};
use crates_io_api::SyncClient;
use reqwest::Url;

use crate::extract_commit_hash;

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
