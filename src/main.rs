use std::path::PathBuf;

use anyhow::Context;
use tracing_subscriber::{fmt::SubscriberBuilder, EnvFilter};

pub mod config;
pub mod profile;
pub mod sandbox;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    SubscriberBuilder::default()
        .with_env_filter(EnvFilter::from_default_env())
        .without_time()
        .init();

    let profile = profile::Profile::load("./typster.toml").await?;

    let root_dir = PathBuf::from("./typster");
    tokio::fs::create_dir_all(&root_dir)
        .await
        .context("failed to create root directory")?;

    let root_dir = tokio::fs::canonicalize(root_dir)
        .await
        .context("failed to canonicalize root directory")?;

    let samples = tokio::fs::canonicalize("./samples")
        .await
        .context("failed to canonicalize samples directory")?;

    let sandbox = sandbox::Sandbox::new(
        &profile,
        &root_dir,
        "https://github.com/Dherse/typst",
        "instrumentation",
    )
    .await?;

    dbg!(sandbox.clone(&profile).await?);
    dbg!(sandbox.fetch(&profile).await?);
    dbg!(sandbox.build(&profile).await?);
    dbg!(sandbox.bench_e2e(&profile, &samples).await?);

    Ok(())
}
