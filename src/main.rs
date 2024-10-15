use clap::Parser;
use retrospective_crate_version_tagging::detect::DetectMissingTags;

#[derive(clap::Parser)]
enum Cli {
    Detect(DetectMissingTags),
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
            serde_json::to_writer_pretty(std::io::stdout().lock(), &result).unwrap();
        }
    }
}
