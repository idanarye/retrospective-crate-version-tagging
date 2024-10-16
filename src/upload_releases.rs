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
        versions.sort_by_key(|version| version.created_at);
        // println!("{:?}", versions);
        for version in versions.into_iter() {
            let VersionToTag {
                version: _,
                tagname,
                commit_hash,
                created_at: _,
                title,
                notes,
            } = &version;
            let mut cmd = std::process::Command::new("gh");
            cmd.args(["release", "create", tagname]);
            cmd.args(["--target", commit_hash]);
            cmd.args(["--title", title]);
            cmd.args(["--notes", notes]);
            if self.dry_run {
                println!("{:?}", cmd);
            } else {
                tracing::info!(tagname = tagname, title = title, "Uploading with gh");
                cmd.spawn()?.wait()?;
            }
        }
        Ok(())
    }
}
