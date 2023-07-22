use anyhow::{Context, bail};
use bollard::Docker;
use futures_util::StreamExt;
use lapin::{
    options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties,
};
use sandbox::ContainerOutput;
use tracing::warn;
use tracing_subscriber::{fmt::SubscriberBuilder, EnvFilter};
use typster_proto::{
    deserialize_from, serialize, BenchQuery, BenchResult, BenchSamples, StageOutput, BenchWalltimeSamples,
};

use crate::results::{Metric, SamplingResults};

pub mod config;
pub mod profile;
pub mod results;
pub mod sandbox;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    SubscriberBuilder::default()
        .with_env_filter(EnvFilter::from_default_env())
        .without_time()
        .init();

    let profile = profile::Profile::load("./typster.toml").await?;
    let root_dir = profile.workdir.clone();
    let samples = profile.samples.clone();

    let docker = Docker::connect_with_local_defaults()?;

    let sandbox = sandbox::Sandbox::new(
        &profile,
        &root_dir,
        "https://github.com/Dherse/typst",
        "e523b076339ea67a9e8cbba983f716fc77fde11d",
        Some("latest".into()),
    )
    .await?;

    let _span = tracing::info_span!("sandbox", id = %sandbox.id);

    let clone_res = sandbox.clone(&profile, &docker).await?;
    if clone_res.exitcode != 0 {
        bail!("Failed to clone repository");
    }

    warn!("Cloned");
    
    let fetch_res = sandbox.fetch(&profile, &docker).await?;
    if fetch_res.exitcode != 0 {
        bail!("Failed to cargo fetch");
    }

    warn!("Fetched");

    let pgo_build_profile = sandbox.pgo_build_profile(&profile, &docker).await?;
    if pgo_build_profile.exitcode != 0 {
        bail!("Failed to build with PGO profile");
    }

    warn!("PGO unprofiled built");

    let pgo_profile = sandbox.pgo_profile(&profile, &docker, &samples).await?;
    if pgo_profile.exitcode != 0 {
        bail!("Failed to profile with PGO");
    }

    warn!("PGO profiled");

    let pgo_build = sandbox.pgo_build(&profile, &docker).await?;
    if pgo_build.exitcode != 0 {
        bail!("Failed to build with PGO");
    }

    warn!("PGO profiled built");

    let bench_res = sandbox.bench_e2e(&profile, &docker, &samples, false, true).await?;
    if bench_res.exitcode != 0 {
        bail!("Failed to bench");
    }

    warn!("Bench PGO");

    let build = sandbox.build(&profile, &docker).await?;
    if build.exitcode != 0 {
        bail!("Failed to build");
    }

    warn!("Built");

    let bench_res = sandbox.bench_e2e(&profile, &docker, &samples, false, false).await?;
    if bench_res.exitcode != 0 {
        bail!("Failed to bench");
    }

    warn!("Bench normal");
   /*let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());
    let conn = Connection::connect(
        &addr,
        ConnectionProperties::default()
            .with_executor(tokio_executor_trait::Tokio::current())
            .with_reactor(tokio_reactor_trait::Tokio),
    )
    .await?;
    tracing::info!("Connected to AMQP broker");

    let channel_b = conn.create_channel().await?;
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        let data = serialize(&BenchQuery {
            id: "typst-main".into(),
            repo: "https://github.com/Typst/typst".into(),
            commit: "1fa56a317c8d8da76546e6b5e19b279d84380837".into(),
        })
        .unwrap();

        channel_b
            .basic_publish(
                "",
                "bench",
                BasicPublishOptions::default(),
                &data,
                BasicProperties::default(),
            )
            .await
            .unwrap()
            .await
            .unwrap();
    });

    let channel_c = conn.create_channel().await?;
    let rx = tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;

        let mut incoming_results = channel_c
            .basic_consume(
                "results",
                "test-readout",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .unwrap();

        while let Some(message) = incoming_results.next().await {
            let delivery = message.expect("Failed to get message");
            delivery
                .ack(BasicAckOptions::default())
                .await
                .expect("Failed to ack message");

            let bench_result: BenchResult = deserialize_from(&delivery.data[..])
                .context("Failed to deserialize bench result")
                .unwrap();

            tracing::info!("{:#?}", bench_result);
        }
    });

    let channel = conn.create_channel().await?;
    channel
        .queue_declare(
            "bench",
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    channel
        .queue_declare(
            "results",
            QueueDeclareOptions {
                durable: true,
                ..Default::default()
            },
            FieldTable::default(),
        )
        .await?;

    let mut incoming_benches = channel
        .basic_consume(
            "bench",
            "bencher",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    while let Some(message) = incoming_benches.next().await {
        let delivery = message.context("Failed to get message")?;
        delivery
            .ack(BasicAckOptions::default())
            .await
            .context("Failed to ack message")?;

        let bench_query: BenchQuery =
            deserialize_from(&delivery.data[..]).context("Failed to deserialize bench query")?;

        let sandbox = sandbox::Sandbox::new(
            &profile,
            &root_dir,
            &bench_query.repo,
            &bench_query.commit,
            Some(bench_query.id),
        )
        .await?;

        let _span = tracing::info_span!("sandbox", id = %sandbox.id);
        let clone_res = sandbox.clone(&profile, &docker).await?;
        if clone_res.exitcode != 0 {
            send_result(
                &channel,
                BenchResult::Failure {
                    id: sandbox.id.clone(),
                    stage: "clone".into(),
                    clone: Some(clone_res.into()),
                    fetch: None,
                    build: None,
                    bench_e2e: None,
                    bench_walltime: None,
                },
            )
            .await?;

            continue;
        }

        let fetch_res = sandbox.fetch(&profile, &docker).await?;
        if fetch_res.exitcode != 0 {
            send_result(
                &channel,
                BenchResult::Failure {
                    id: sandbox.id.clone(),
                    stage: "fetch".into(),
                    clone: Some(clone_res.into()),
                    fetch: Some(fetch_res.into()),
                    build: None,
                    bench_e2e: None,
                    bench_walltime: None,
                },
            )
            .await?;

            continue;
        }

        let build_res = sandbox.build(&profile, &docker).await?;
        if build_res.exitcode != 0 {
            send_result(
                &channel,
                BenchResult::Failure {
                    id: sandbox.id.clone(),
                    stage: "build".into(),
                    clone: Some(clone_res.into()),
                    fetch: Some(fetch_res.into()),
                    build: Some(build_res.into()),
                    bench_e2e: None,
                    bench_walltime: None,
                },
            )
            .await?;

            continue;
        }

        let bench_res = sandbox
            .bench_e2e(&profile, &docker, &samples, true)
            .await?;
        if bench_res.exitcode != 0 {
            send_result(
                &channel,
                BenchResult::Failure {
                    id: sandbox.id.clone(),
                    stage: "bench_e2e".into(),
                    clone: Some(clone_res.into()),
                    fetch: Some(fetch_res.into()),
                    build: Some(build_res.into()),
                    bench_e2e: Some(bench_res.into()),
                    bench_walltime: None,
                },
            )
            .await?;

            continue;
        }

        let mut procinfo = Vec::new();
        for sample in profile.samples.to_results_file(&sandbox.results) {
            tracing::info!("opening sample file: {}", sample.display());
            let file = tokio::fs::File::open(&sample)
                .await
                .context("failed to open bench output file")?
                .into_std()
                .await;

            let parsed: SamplingResults = serde_json::from_reader(&file)
                .context("failed to parse bench output file")?;

            procinfo.push(BenchSamples {
                name: sample.file_name().unwrap().to_string_lossy().into(),
                user_time: parsed
                    .samples
                    .iter()
                    .find(|s| s.metric == Metric::UserCpuTime)
                    .context("missing user CPU time")?
                    .samples
                    .clone(),
                system_time: parsed
                    .samples
                    .iter()
                    .find(|s| s.metric == Metric::SystemCpuTime)
                    .context("missing system CPU time")?
                    .samples
                    .clone(),
                virtual_memory: parsed
                    .samples
                    .iter()
                    .find(|s| s.metric == Metric::VirtualMemory)
                    .context("missing virtual memory")?
                    .samples
                    .clone(),
                resident_memory: parsed
                    .samples
                    .iter()
                    .find(|s| s.metric == Metric::ResidentMemory)
                    .context("missing resident memory")?
                    .samples
                    .clone(),
                cpu_percent: parsed
                    .samples
                    .iter()
                    .find(|s| s.metric == Metric::Load)
                    .context("missing CPU load")?
                    .samples
                    .clone(),
            });
        }

        let bench_walltime = sandbox
            .bench_walltime(&profile, &docker, &samples, true)
            .await?;
        if bench_walltime.exitcode != 0 {
            send_result(
                &channel,
                BenchResult::Failure {
                    id: sandbox.id.clone(),
                    stage: "bench_walltime".into(),
                    clone: Some(clone_res.into()),
                    fetch: Some(fetch_res.into()),
                    build: Some(build_res.into()),
                    bench_e2e: Some(bench_res.into()),
                    bench_walltime: Some(bench_walltime.into()),
                },
            )
            .await?;

            continue;
        }

        let mut walltimes = Vec::new();
        for sample in profile.samples.to_results_file(&sandbox.walltimes) {
            tracing::info!("opening walltime sample file: {}", sample.display());
            let file = tokio::fs::File::open(&sample)
                .await
                .context("failed to open walltime output file")?
                .into_std()
                .await;


            let parsed: SamplingResults = serde_json::from_reader(&file)
                .context("failed to parse bench output file")?;

            walltimes.push(BenchWalltimeSamples {
                name: sample.file_name().unwrap().to_string_lossy().into(),
                walltime: parsed
                    .samples
                    .iter()
                    .find(|s| s.metric == Metric::Time)
                    .context("missing wall time")?
                    .samples
                    .clone(),
            });
        }

        send_result(
            &channel,
            BenchResult::Success {
                id: sandbox.id.clone(),
                clone: clone_res.into(),
                fetch: fetch_res.into(),
                build: build_res.into(),
                bench_e2e: bench_res.into(),
                bench_walltime: bench_walltime.into(),
                samples: procinfo,
                walltimes,
            },
        )
        .await?;
    }

    rx.await?;*/

    Ok(())
}

async fn send_result(channel: &Channel, result: BenchResult) -> anyhow::Result<()> {
    let data = typster_proto::serialize(&result)?;
    channel
        .basic_publish(
            "",
            "results",
            BasicPublishOptions::default(),
            &data,
            BasicProperties::default(),
        )
        .await
        .context("Failed to publish")?
        .await
        .context("Failed to wait for confirmation")?;

    Ok(())
}

impl From<ContainerOutput> for StageOutput {
    fn from(output: ContainerOutput) -> Self {
        Self {
            exitcode: output.exitcode as _,
            stdout: output.stdout,
            stderr: output.stderr,
        }
    }
}
