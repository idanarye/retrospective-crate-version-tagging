use clap::Parser;
use retrospective_crate_version_tagging::CrateVersion;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    crate_name: String,
}

fn main() {
    let cli = Cli::parse();
    let versions = CrateVersion::for_crate(&cli.crate_name).unwrap();
    println!("{:?}", versions.iter().map(|v| v.name.clone()).collect::<Vec<_>>());
    println!("{:?}", versions[0].resolve_commit_hash());
}
