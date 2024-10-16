use clap::Parser;
use retrospective_crate_version_tagging::{
    detect::DetectMissingTags, upload_releases::UploadReleases,
};

#[derive(clap::Parser)]
enum Cli {
    Detect(DetectMissingTags),
    Upload(UploadReleases),
}

fn main() {
    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt()
            .with_writer(std::io::stderr)
            .with_env_filter(
                tracing_subscriber::EnvFilter::builder()
                    .with_default_directive(tracing::Level::WARN.into())
                    .from_env()
                    .unwrap(),
            )
            .finish(),
    )
    .unwrap();
    match Cli::parse() {
        Cli::Detect(detect_missing_tags) => {
            let result = detect_missing_tags.run().unwrap();
            serde_yml::to_writer(std::io::stdout().lock(), &result).unwrap();
        }
        Cli::Upload(upload_releases) => {
            upload_releases
                .upload_versions_as_releases(
                    serde_yml::from_reader(std::io::stdin().lock()).unwrap(),
                )
                .unwrap();
        }
    }
}
