use std::path::PathBuf;

use anyhow::Context;
use tracing_subscriber::{fmt::SubscriberBuilder, EnvFilter};

pub mod config;
pub mod sandbox;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    SubscriberBuilder::default()
        .with_env_filter(EnvFilter::from_default_env())
        .without_time()
        .init();

    let root_dir = PathBuf::from("./typster");
    tokio::fs::create_dir_all(&root_dir).await.context("failed to create root directory")?;
    let root_dir = tokio::fs::canonicalize(root_dir).await.context("failed to canonicalize root directory")?;

    let sandbox = sandbox::Sandbox::new(
        &root_dir,
        "https://github.com/Dherse/typst",
        "content-rework",
    ).await?;

    dbg!(sandbox.clone().await?);
    dbg!(sandbox.fetch().await?);
    dbg!(sandbox.build().await?);

    Ok(())
}
