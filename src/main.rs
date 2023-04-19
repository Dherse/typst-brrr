use std::path::PathBuf;

use anyhow::Context;
use bollard::Docker;
use build_images::build_images;
use clap::Parser;
use tokio::fs::File;

pub mod build_images;
pub mod config;
pub mod pipeline;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let docker = Docker::connect_with_local_defaults()?;

    let images = build_images(&docker).await?;
    eprintln!("Built images");

    let git_dir = PathBuf::from("./_git/");
    let cargo_dir = PathBuf::from("./_cargo/");
    let data_dir = PathBuf::from("./_data/");
    let sample_dir = PathBuf::from("./samples/");

    tokio::fs::create_dir_all(&git_dir)
        .await
        .context("failed to create git dir")?;

    tokio::fs::create_dir_all(&cargo_dir)
        .await
        .context("failed to create cargo dir")?;

    tokio::fs::create_dir_all(&data_dir)
        .await
        .context("failed to create data dir")?;

    tokio::fs::create_dir_all(&sample_dir)
        .await
        .context("failed to create sample dir")?;

    images
        .run_clone(&"https://github.com/Dherse/typst", "content-rework", &git_dir)
        .await?;
    eprintln!("cloned");

    images
        .run_fetch(&git_dir, &cargo_dir)
        .await
        .context("failed to run fetch")?;
    eprintln!("fetched");

    images
        .run_build(&git_dir, &cargo_dir)
        .await
        .context("failed to run fetch")?;
    eprintln!("built");

    images
        .run_e2e_benches(&git_dir, &cargo_dir, &sample_dir, &data_dir, &[ "/samples/conformal_prediction/conformal_prediction.typ"])
        .await
        .context("failed to run e2e benches")?;
    eprintln!("ran e2e benches");

    Ok(())
}
