use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

use anyhow::Context;
use bollard::{
    container::{Config, CreateContainerOptions},
    image::BuildImageOptions,
    service::{HostConfig, MountTypeEnum},
};
use futures_util::StreamExt;
use rand::Rng;
use tokio::{task::JoinHandle, time::timeout};

pub struct Docker(bollard::Docker);

impl Docker {
    pub fn new() -> anyhow::Result<Self> {
        let docker = bollard::Docker::connect_with_local_defaults()
            .context("failed to connect to Docker daemon")?;
        Ok(Self(docker))
    }

    pub async fn create_image<S: AsRef<str>>(
        &self,
        name: S,
        tar_ball: Vec<u8>,
    ) -> anyhow::Result<Image> {
        let build_options = BuildImageOptions {
            dockerfile: "dockerfile",
            t: name.as_ref(),
            ..Default::default()
        };

        // Build the image, this returns a stream of messages regarding the building of the image
        let mut stream = self
            .0
            .build_image(build_options, None, Some(tar_ball.into()));

        // Iterate over the stream and print the messages
        while let Some(message) = stream.next().await {
            let message = message.context("error building image")?;
            if let Some(status) = message.status {
                tracing::info!("{status}");
            }

            if let Some(error) = message.error {
                if let Some(details) = message.error_detail {
                    tracing::error!(
                        "{error}: #{} {}",
                        details.code.unwrap_or(0),
                        details.message.as_deref().unwrap_or("")
                    );
                } else {
                    tracing::error!("{error}");
                }
            }

            if let Some(progress) = message.progress {
                tracing::info!("{progress}");
            }
        }

        Ok(Image(self, name.as_ref().to_string()))
    }

    pub async fn run_container(
        &self,
        image: &Image<'_>,
        env: Vec<String>,
        mounts: Vec<Mount>,
    ) -> anyhow::Result<()> {
        let host_config = HostConfig {
            auto_remove: Some(true),
            privileged: Some(false),
            memory: Some(4 << 30),
            cpuset_cpus: Some("3".to_string()),
            mounts: Some(
                mounts
                    .into_iter()
                    .map(|m| {
                        Ok(bollard::models::Mount {
                            source: Some(
                                m.source
                                    .canonicalize()
                                    .context("failed to cannonicalize source directory")?
                                    .to_str()
                                    .context("malformed source path")?
                                    .to_string(),
                            ),
                            target: Some(
                                m.target
                                    .to_str()
                                    .context("malformed target path")?
                                    .to_string(),
                            ),
                            typ: Some(MountTypeEnum::BIND),
                            ..Default::default()
                        })
                    })
                    .collect::<anyhow::Result<Vec<_>>>()?,
            ),
            ..Default::default()
        };

        let config = Config {
            image: Some(image.1.clone()),
            host_config: Some(host_config),
            env: Some(env),
            ..Default::default()
        };

        let name = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(30)
            .map(char::from)
            .collect::<String>();

        let response = self
            .0
            .create_container(
                Some(CreateContainerOptions {
                    name: &name,
                    platform: None,
                }),
                config,
            )
            .await
            .context("failed to create container")?;

        let id = response.id;
        tracing::info!(id = id, name = name, image = image.1, "Created container");
        for warn in response.warnings {
            tracing::warn!("{}", warn.trim());
        }

        self.0
            .start_container::<&str>(&id, None)
            .await
            .context("failed to start container")?;

        let attach_options = bollard::container::AttachContainerOptions::<String> {
            stream: Some(true),
            stdout: Some(true),
            stderr: Some(true),
            stdin: Some(false),
            logs: Some(true),
            ..Default::default()
        };

        let mut output = self
            .0
            .attach_container(&id, Some(attach_options))
            .await
            .context("failed to attach to container")?
            .output;

        let is_stopped = Arc::new(AtomicBool::new(false));
        let is_stopped2 = Arc::clone(&is_stopped);
        let logging: JoinHandle<anyhow::Result<()>> = tokio::spawn(async move {
            loop {
                let res = timeout(Duration::from_millis(20), output.next()).await;
                if let Ok(Some(out)) = res {
                    let out = out.context("failed to read container output")?;
                    tracing::info!("{}", out.to_string().trim());
                } else {
                    if is_stopped2.load(Ordering::Relaxed) {
                        break Ok(());
                    }
                }
            }
        });

        let mut wait_stream = self.0.wait_container::<String>(&id, None);
        while let Some(out) = wait_stream.next().await {
            out.context("failed to wait for container")?;
        }

        is_stopped.store(true, Ordering::Relaxed);

        logging.await??;

        Ok(())
    }
}

pub struct Image<'a>(&'a Docker, String);

impl<'a> Image<'a> {
    pub async fn run_container(&self, env: Vec<String>, mounts: Vec<Mount>) -> anyhow::Result<()> {
        self.0.run_container(self, env, mounts).await
    }
}

pub struct Mount {
    pub source: PathBuf,
    pub target: PathBuf,
}

impl Mount {
    pub fn new<P1: AsRef<Path>, P2: AsRef<Path>>(source: P1, target: P2) -> Self {
        Self {
            source: source.as_ref().into(),
            target: target.as_ref().into(),
        }
    }
}
