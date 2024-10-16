use clap::Parser;
use retrospective_crate_version_tagging::{
    detect::DetectMissingTags, upload_releases::UploadReleases,
};
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(clap::Parser)]
enum Cli {
    Detect(DetectMissingTags),
    Upload(UploadReleases),
}

fn main() {
    let indicatif_layer = IndicatifLayer::new();
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_writer(indicatif_layer.get_stderr_writer()))
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_default_directive(tracing::Level::WARN.into())
                .from_env()
                .unwrap(),
        )
        .with(indicatif_layer)
        .init();
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
