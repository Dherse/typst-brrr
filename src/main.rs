use std::path::PathBuf;

use anyhow::Context;
use build_images::{BENCH_END_TO_END, BUILD_TAR, CLONE_TAR, FETCH_TAR};
use docker::Mount;
use tracing::Instrument;
use tracing_subscriber::{filter::EnvFilter, fmt::SubscriberBuilder};

use crate::docker::Docker;

pub mod build_images;
pub mod config;
pub mod docker;
pub mod pipeline;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    SubscriberBuilder::default()
        .with_env_filter(EnvFilter::from_default_env())
        .without_time()
        .init();

    let docker = Docker::new()?;

    let clone = docker
        .create_image("typst-clone", CLONE_TAR.to_vec())
        .await?;

    let fetch = docker
        .create_image("typst-fetch", FETCH_TAR.to_vec())
        .await?;
    
    let build = docker
        .create_image("typst-build", BUILD_TAR.to_vec())
        .await?;
    let bench = docker
        .create_image("typst-bench", BENCH_END_TO_END.to_vec())
        .await?;

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

    clone
        .run_container(
            vec![
                format!("REPO_URL={}", "https://github.com/Dherse/typst"),
                format!("COMMIT={}", "content-rework"),
            ],
            vec![Mount::new(&git_dir, "/data")],
        )
        .instrument(tracing::info_span!("clone"))
        .await?;

    fetch
        .run_container(
            vec![],
            vec![
                Mount::new(&git_dir, "/data"),
                Mount::new(&cargo_dir, "/cargo"),
            ],
        )
        .instrument(tracing::info_span!("fetch"))
        .await?;

    build
        .run_container(
            vec![],
            vec![
                Mount::new(&git_dir, "/data"),
                Mount::new(&cargo_dir, "/cargo"),
            ],
        )
        .instrument(tracing::info_span!("build"))
        .await?;

    bench
        .run_container(
            vec![
                format!(
                    "FILE_LIST={}",
                    "/samples/conformal_prediction/conformal_prediction.typ"
                ),
                format!("WARMUPS={}", 5),
                format!("RUNS={}", 200),
            ],
            vec![
                Mount::new(git_dir.join("target").join("release"), "/binary"),
                Mount::new(&cargo_dir, "/cargo"),
                Mount::new(&sample_dir, "/samples"),
                Mount::new(&data_dir, "/data"),
            ],
        )
        .instrument(tracing::info_span!("bench"))
        .await?;

    Ok(())
}
