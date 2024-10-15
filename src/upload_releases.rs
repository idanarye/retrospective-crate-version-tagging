use crate::detect::VersionToTag;

#[derive(clap::Args)]
#[command(about, long_about = None)]
pub struct UploadReleases {
    #[arg(long, action)]
    dry_run: bool,
}

impl UploadReleases {
    pub fn upload_versions_as_releases(
        &self,
        mut versions: Vec<VersionToTag>,
    ) -> anyhow::Result<()> {
        versions.sort_by_key(|version| version.version.clone()); // TODO: sort by creation time
        for version in versions {
            println!("{:?}", version.version);
        }
        Ok(())
    }
}
