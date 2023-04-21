use std::{
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration, ops::{Deref, Not},
};

use anyhow::Context;
use bollard::{
    container::{Config, CreateContainerOptions, NetworkingConfig},
    image::BuildImageOptions,
    service::{HostConfig, MountTypeEnum}, errors::Error,
};
use futures_util::StreamExt;
use rand::Rng;
use tokio::{task::JoinHandle, time::timeout};

pub struct Docker(bollard::Docker);

impl Deref for Docker {
    type Target = bollard::Docker;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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

        Ok(Image {
            docker: self,
            name: name.as_ref().to_string(),
            network: None,
        })
    }

    pub async fn create_network(&self, name: String, internal: bool) -> anyhow::Result<Network> {
        let network = self
            .0
            .create_network(bollard::network::CreateNetworkOptions {
                name: name.clone(),
                internal,
                ..Default::default()
            })
            .await
            .context("failed to create network")?;

        if let Some(warning) = network.warning {
            tracing::warn!("{}", warning.trim());
        }

        Ok(Network {
            docker: self,
            name,
            id: network.id.context("no id returned for network")?,
        })
    }
}

pub struct Network<'a> {
    docker: &'a Docker,
    name: String,
    id: String,
}

pub struct Image<'a> {
    docker: &'a Docker,
    name: String,
    network: Option<Network<'a>>,
}

impl<'a> Image<'a> {
    pub fn with_network(mut self, network: Network<'a>) -> Self {
        self.network = Some(network);
        self
    }

    pub async fn run_container(&self, env: Vec<String>, mounts: Vec<Mount>) -> anyhow::Result<()> {
        let host_config = HostConfig {
            auto_remove: Some(true),
            privileged: Some(false),
            memory: Some(4 << 30),
            cpuset_cpus: Some("3".to_string()),
            network_mode: Some(self.network.as_ref().map_or_else(|| "none".to_string(), |n| n.id.clone())),
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
            image: Some(self.name.clone()),
            host_config: Some(host_config),
            env: Some(env),
            network_disabled: Some(self.network.is_none()),
            ..Default::default()
        };

        let name = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(30)
            .map(char::from)
            .collect::<String>();

        let response = self
            .docker
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
        tracing::info!(id = id, name = name, image = self.name, "Created container");
        for warn in response.warnings {
            tracing::warn!("{}", warn.trim());
        }

        self.docker
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
            .docker
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

        let mut wait_stream = self.docker.wait_container::<String>(&id, None);
        while let Some(out) = wait_stream.next().await {
            if let Err(err) = out {
                match err {
                    Error::DockerResponseServerError { status_code, .. }  if status_code == 404 => {
                        tracing::warn!("Container exited before we could wait for it");
                        break;
                    },
                    _ => {
                        return Err(err).context("failed to wait for container");
                    }
                }
            }
        }

        is_stopped.store(true, Ordering::Relaxed);

        logging.await??;

        Ok(())
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
