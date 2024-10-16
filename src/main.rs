use clap::Parser;
use retrospective_crate_version_tagging::{
    create_releases::CreateReleases, detect::DetectMissingTags,
};
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(clap::Parser)]
enum Cli {
    Detect(DetectMissingTags),
    CreateReleases(CreateReleases),
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
            // Avoid printing empty values, because they get printed as `[]` which ruins the YAML
            // concatanation
            if !result.is_empty() {
                serde_yml::to_writer(std::io::stdout().lock(), &result).unwrap();
            }
        }
        Cli::CreateReleases(create_releases) => {
            create_releases
                .create_releases_from_versions(
                    serde_yml::from_reader(std::io::stdin().lock()).unwrap(),
                )
                .unwrap();
        }
    }
}
