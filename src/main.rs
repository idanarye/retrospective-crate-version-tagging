use std::io::prelude::*;

use clap::Parser;
use retrospective_crate_version_tagging::{
    detect::{DetectMissingTags, VersionToTag},
    upload_releases::UploadReleases,
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
            let mut stdout = std::io::stdout().lock();
            serde_json::to_writer_pretty(&mut stdout, &result).unwrap();
            writeln!(&mut stdout).unwrap();
        }
        Cli::Upload(upload_releases) => {
            let deserializer = serde_json::Deserializer::from_reader(std::io::stdin().lock());
            upload_releases
                .upload_versions_as_releases(
                    deserializer
                        .into_iter::<Vec<VersionToTag>>()
                        .flat_map(|versions| versions.unwrap())
                        .collect(),
                )
                .unwrap();
        }
    }
}
