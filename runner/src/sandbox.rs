use std::{
    os::unix::prelude::PermissionsExt,
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::Context;
use bollard::{
    container::{
        AttachContainerOptions, AttachContainerResults, CreateContainerOptions, LogOutput,
        RemoveContainerOptions, StopContainerOptions,
    },
    service::{HostConfig, MountTypeEnum, RestartPolicy, RestartPolicyNameEnum},
    Docker,
};
use futures_util::StreamExt;
use rand::Rng;
use tokio::{time::timeout, fs::create_dir};

use crate::profile::{Profile, Samples, Stage};

pub struct Sandbox {
    pub id: String,
    pub delete_on_exit: bool,

    pub parent: PathBuf,
    pub git: PathBuf,
    pub cargo: PathBuf,
    pub results: PathBuf,
    pub walltimes: PathBuf,
    pub pgo_data: PathBuf,

    pub repository: String,
    pub commit: String,

    pub pipe: bool,
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        if self.delete_on_exit {
            if let Err(e) = std::fs::remove_dir_all(&self.git) {
                tracing::error!("failed to remove git directory: {}", e);
            }

            if let Err(e) = std::fs::remove_dir_all(&self.cargo) {
                tracing::error!("failed to remove cargo directory: {}", e);
            }

            if let Err(e) = std::fs::remove_dir_all(&self.results) {
                tracing::error!("failed to remove results directory: {}", e);
            }

            if let Err(e) = std::fs::remove_dir_all(&self.walltimes) {
                tracing::error!("failed to remove walltimes directory: {}", e);
            }

            if let Err(e) = std::fs::remove_dir_all(&self.pgo_data) {
                tracing::error!("failed to remove pgo-data directory: {}", e);
            }
        }
    }
}

async fn create_directory(parent: impl AsRef<Path>, id: &str) -> anyhow::Result<PathBuf> {
    let path = parent.as_ref().join(id);

    tokio::fs::create_dir_all(&path)
        .await
        .context("failed to create directory")?;

    tokio::fs::set_permissions(&path, wide_open_permissions())
        .await
        .context("failed to set permissions on directory")?;

    Ok(path)
}

impl Sandbox {
    pub async fn new<P: AsRef<Path>, S1: ToString, S2: ToString>(
        profile: &Profile,
        root: P,
        repository: S1,
        commit: S2,
        id: Option<String>,
    ) -> anyhow::Result<Self> {
        //generate random ID of length 10 using the rand crate
        let id = id.unwrap_or_else(|| {
            rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(10)
                .map(char::from)
                .collect::<String>()
        });

        let parent = create_directory(root, &id).await?;
        let git = create_directory(&parent, "git").await?;
        let cargo = create_directory(&parent, "cargo").await?;
        let results = create_directory(&parent, "results").await?;
        let walltimes = create_directory(&parent, "walltimes").await?;
        let pgo_data = create_directory(&parent, "pgo-data").await?;

        Ok(Self {
            id,
            delete_on_exit: profile.delete_on_exit,
            parent,
            git,
            cargo,
            results,
            walltimes,
            pgo_data,
            repository: repository.to_string(),
            commit: commit.to_string(),
            pipe: cfg!(debug_assertions),
        })
    }

    /// Returns the ID of the sandbox
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the path to the results directory
    pub fn results(&self) -> &Path {
        &self.results
    }

    /// Returns the path to the walltimes directory
    pub fn walltimes(&self) -> &Path {
        &self.walltimes
    }

    /// Returns the path to the cargo directory
    pub fn cargo(&self) -> &Path {
        &self.cargo
    }

    /// Returns the path to the git directory
    pub fn git(&self) -> &Path {
        &self.git
    }

    /// Returns the repository URL
    pub fn repo(&self) -> &str {
        &self.repository
    }

    /// Returns the commit hash
    pub fn commit(&self) -> &str {
        &self.commit
    }

    /// Clones the repository into the git directory
    pub async fn clone(
        &self,
        profile: &Profile,
        docker: &Docker,
    ) -> anyhow::Result<ContainerOutput> {
        let stage = &profile.stages.clone;

        let repository = format!("REPO_URL={}", self.repository);
        let commit = format!("COMMIT={}", self.commit);
        let container = create_safe_container(
            docker,
            stage,
            vec![&repository, &commit],
            vec![Mount {
                target: "/typster".into(),
                source: self.git.clone(),
                read_only: false,
            }],
        )
        .await?;

        let output = container.join(self.pipe).await?;

        if output.exitcode != 0 {
            tracing::error!("failed to clone repository");
        }

        Ok(output)
    }

    /// Fetches the crates into the cargo directory
    pub async fn fetch(
        &self,
        profile: &Profile,
        docker: &Docker,
    ) -> anyhow::Result<ContainerOutput> {
        let stage = &profile.stages.fetch;

        let container = create_safe_container(
            docker,
            stage,
            vec![],
            vec![
                Mount {
                    target: "/typster".into(),
                    source: self.git.clone(),
                    read_only: true,
                },
                Mount {
                    target: "/cargo".into(),
                    source: self.cargo.clone(),
                    read_only: false,
                },
            ],
        )
        .await?;

        let output = container.join(self.pipe).await?;

        if output.exitcode != 0 {
            tracing::error!("failed to fetch crates");
        }

        Ok(output)
    }

    /// Builds the project
    pub async fn build(
        &self,
        profile: &Profile,
        docker: &Docker,
    ) -> anyhow::Result<ContainerOutput> {
        let stage = &profile.stages.build;

        let container = create_safe_container(
            docker,
            stage,
            vec![],
            vec![
                Mount {
                    target: "/typster".into(),
                    source: self.git.clone(),
                    read_only: false,
                },
                Mount {
                    target: "/cargo".into(),
                    source: self.cargo.clone(),
                    read_only: true,
                },
            ],
        )
        .await?;

        let output = container.join(self.pipe).await?;

        if output.exitcode != 0 {
            tracing::error!("failed to build project");
        }

        Ok(output)
    }

    /// Runs the benchmarks
    pub async fn bench_e2e(
        &self,
        profile: &Profile,
        docker: &Docker,
        samples: &Samples,
        main: bool,
    ) -> anyhow::Result<ContainerOutput> {
        let stage = &profile.stages.bench_e2e;

        let settings = if main {
            &profile.profiles.main
        } else {
            &profile.profiles.other
        };

        let interval: Duration = settings.interval.into();
        let sleep: Duration = settings.sleep.into();
        let env_warmups = format!("WARMUPS={}", settings.warmups);
        let env_runs = format!("RUNS={}", settings.runs);
        let env_samples = format!(
            "FILE_LIST={}",
            samples.to_env().context("no samples found")?
        );
        let env_freq = format!("FREQUENCY={}", interval.as_micros());
        let env_work = format!("WORK={}", settings.work);
        let env_sleep = format!("SLEEP={}", sleep.as_millis());
        let container = create_safe_container(
            docker,
            stage,
            vec![
                &env_warmups,
                &env_runs,
                &env_samples,
                &env_freq,
                &env_work,
                &env_sleep,
            ],
            vec![
                Mount {
                    target: "/typster".into(),
                    source: self.git.clone(),
                    read_only: true,
                },
                Mount {
                    target: "/samples".into(),
                    source: tokio::fs::canonicalize(&samples.root)
                        .await
                        .context("failed to canonicalize path")?,
                    read_only: true,
                },
                Mount {
                    target: "/data".into(),
                    source: self.results.clone(),
                    read_only: false,
                },
            ],
        )
        .await?;

        let output = container.join(self.pipe).await?;

        if output.exitcode != 0 {
            tracing::error!("failed to bench project");
        }

        Ok(output)
    }

    /// Runs the benchmarks
    pub async fn bench_walltime(
        &self,
        profile: &Profile,
        docker: &Docker,
        samples: &Samples,
        main: bool,
    ) -> anyhow::Result<ContainerOutput> {
        let stage = &profile.stages.bench_walltime;

        let settings = if main {
            &profile.profiles.main
        } else {
            &profile.profiles.other
        };

        let sleep: Duration = settings.sleep.into();
        let env_warmups = format!("WARMUPS={}", settings.warmups);
        let env_runs = format!("RUNS={}", settings.runs);
        let env_samples = format!(
            "FILE_LIST={}",
            samples.to_env().context("no samples found")?
        );
        let env_work = format!("WORK={}", settings.work);
        let env_sleep = format!("SLEEP={}", sleep.as_millis());
        let container = create_safe_container(
            docker,
            stage,
            vec![
                &env_warmups,
                &env_runs,
                &env_samples,
                &env_work,
                &env_sleep,
            ],
            vec![
                Mount {
                    target: "/typster".into(),
                    source: self.git.clone(),
                    read_only: true,
                },
                Mount {
                    target: "/samples".into(),
                    source: tokio::fs::canonicalize(&samples.root)
                        .await
                        .context("failed to canonicalize path")?,
                    read_only: true,
                },
                Mount {
                    target: "/data".into(),
                    source: self.walltimes.clone(),
                    read_only: false,
                },
            ],
        )
        .await?;

        let output = container.join(self.pipe).await?;

        if output.exitcode != 0 {
            tracing::error!("failed to bench project");
        }

        Ok(output)
    }

    /// Builds the project
    pub async fn pgo_build_profile(
        &self,
        profile: &Profile,
        docker: &Docker,
    ) -> anyhow::Result<ContainerOutput> {
        let stage = &profile.stages.pgo_build_profile;

        let container = create_safe_container(
            docker,
            stage,
            vec![],
            vec![
                Mount {
                    target: "/typster".into(),
                    source: self.git.clone(),
                    read_only: false,
                },
                Mount {
                    target: "/cargo".into(),
                    source: self.cargo.clone(),
                    read_only: true,
                },
                Mount {
                    target: "/pgo-data".into(),
                    source: self.pgo_data.clone(),
                    read_only: false,
                },
            ],
        )
        .await?;

        let output = container.join(self.pipe).await?;

        if output.exitcode != 0 {
            tracing::error!("failed to build project");
        }

        Ok(output)
    }

    /// Builds the project
    pub async fn pgo_profile(
        &self,
        profile: &Profile,
        docker: &Docker,
        samples: &Samples,
    ) -> anyhow::Result<ContainerOutput> {
        let stage = &profile.stages.pgo_profile;

        let env_samples = format!(
            "FILE_LIST={}",
            samples.to_env().context("no samples found")?
        );
        let container = create_safe_container(
            docker,
            stage,
            vec![
                &env_samples,
            ],
            vec![
                Mount {
                    target: "/typster".into(),
                    source: self.git.clone(),
                    read_only: true,
                },
                Mount {
                    target: "/samples".into(),
                    source: tokio::fs::canonicalize(&samples.root)
                        .await
                        .context("failed to canonicalize path")?,
                    read_only: true,
                },
                Mount {
                    target: "/data".into(),
                    source: self.walltimes.clone(),
                    read_only: false,
                },
                Mount {
                    target: "/pgo-data".into(),
                    source: self.pgo_data.clone(),
                    read_only: false,
                }
            ],
        )
        .await?;

        let output = container.join(self.pipe).await?;

        if output.exitcode != 0 {
            tracing::error!("failed to bench project");
        }

        Ok(output)
    }

    /// Builds the project
    pub async fn pgo_build(
        &self,
        profile: &Profile,
        docker: &Docker,
    ) -> anyhow::Result<ContainerOutput> {
        let stage = &profile.stages.pgo_build;

        let container = create_safe_container(
            docker,
            stage,
            vec![],
            vec![
                Mount {
                    target: "/typster".into(),
                    source: self.git.clone(),
                    read_only: false,
                },
                Mount {
                    target: "/cargo".into(),
                    source: self.cargo.clone(),
                    read_only: true,
                },
                Mount {
                    target: "/pgo-data".into(),
                    source: self.pgo_data.clone(),
                    read_only: false,
                }
            ],
        )
        .await?;

        let output = container.join(self.pipe).await?;

        if output.exitcode != 0 {
            tracing::error!("failed to build project");
        }

        Ok(output)
    }
}

// We must create a world-writable files so that the process inside
// the Docker container can write into it.
//
// This problem does *not* occur when using the indirection of
// docker-machine.
fn wide_open_permissions() -> std::fs::Permissions {
    PermissionsExt::from_mode(0o777)
}

#[derive(Debug)]
struct Mount {
    source: PathBuf,
    target: PathBuf,
    read_only: bool,
}

struct Container {
    docker: Docker,
    timeout: Duration,
    pub id: String,
    pub stopped: bool,
}

#[derive(Debug)]
pub struct ContainerOutput {
    pub stdout: Vec<String>,
    pub stderr: Vec<String>,
    pub exitcode: i64,
}

impl Container {
    async fn join(mut self, pipe: bool) -> anyhow::Result<ContainerOutput> {
        let options = AttachContainerOptions {
            stream: Some(true),
            logs: Some(true),
            stdout: Some(true),
            stderr: Some(true),
            ..Default::default()
        };

        let docker = self.docker.clone();
        let id = self.id.clone();
        let collect = tokio::spawn(async move {
            let AttachContainerResults { mut output, .. } = docker
                .attach_container::<&str>(&id, Some(options))
                .await
                .context("failed to attach to container")?;

            let mut stdout = Vec::new();
            let mut stderr = Vec::new();

            while let Some(Ok(output)) = output.next().await {
                match output {
                    LogOutput::StdErr { message } => {
                        let message = String::from_utf8_lossy(&message).to_string();
                        for message in message.split('\n') {
                            let trimmed = message.trim();
                            if trimmed.is_empty() {
                                continue;
                            }

                            if pipe {
                                tracing::error!(container = ?id, "{}", trimmed);
                            }
                            stderr.push(trimmed.to_owned());
                        }
                    }
                    LogOutput::StdOut { message } | LogOutput::Console { message } => {
                        let message = String::from_utf8_lossy(&message).to_string();
                        for message in message.split('\n') {
                            let trimmed = message.trim();
                            if trimmed.is_empty() {
                                continue;
                            }

                            if pipe {
                                tracing::info!(container = ?id, "{}", trimmed);
                            }

                            stdout.push(trimmed.to_owned());
                        }
                    }
                    _ => {}
                }
            }

            Ok::<_, anyhow::Error>((stdout, stderr))
        });

        let (stdout, stderr) = timeout(self.timeout, collect)
            .await
            .context("timedout while waiting for container output")?
            .context("failed to join spawned task")?
            .context("failed to collect container output")?;

        let exitcode = self
            .docker
            .inspect_container(&self.id, None)
            .await
            .context("failed to inspect container")?;

        self.stopped = true;

        Ok(ContainerOutput {
            stdout,
            stderr,
            exitcode: exitcode.state.and_then(|s| s.exit_code).unwrap_or(i64::MAX),
        })
    }
}

impl Drop for Container {
    fn drop(&mut self) {
        let stopped = self.stopped;
        let docker = self.docker.clone();
        let id = self.id.clone();
        tokio::spawn(async move {
            if !stopped {
                if let Err(err) = docker
                    .stop_container(&id, Some(StopContainerOptions { t: 10 }))
                    .await
                {
                    tracing::error!("failed to stop container: {}", err);
                }
                return;
            }

            if let Err(err) = docker
                .remove_container(
                    &id,
                    Some(RemoveContainerOptions {
                        force: true,
                        ..Default::default()
                    }),
                )
                .await
            {
                tracing::error!("failed to remove container: {}", err);
            }
        });

        self.stopped = true;
    }
}

#[tracing::instrument(skip(docker))]
async fn create_safe_container(
    docker: &Docker,
    stage: &Stage,
    mut env: Vec<&str>,
    mounts: Vec<Mount>,
) -> anyhow::Result<Container> {
    if stage.networking {
        env.push("HTTP_PROXY=http://172.19.0.2:3128");
        env.push("HTTPS_PROXY=http://172.19.0.2:3128");
        env.push("FTP_PROXY=http://172.19.0.2:3128");
    }

    let mut env = env.clone();

    let timeout_env = format!("TIMEOUT={}", stage.soft_timeout);
    env.push(&timeout_env);

    let secure_config = bollard::container::Config {
        image: Some(&stage.image as &str),
        network_disabled: Some(!stage.networking),
        env: Some(env),
        working_dir: Some("/typster"),
        attach_stderr: Some(true),
        attach_stdout: Some(true),
        tty: Some(true),
        host_config: Some(HostConfig {
            mounts: Some(
                mounts
                    .into_iter()
                    .map(|mount| {
                        Ok(bollard::models::Mount {
                            target: Some(mount.target.display().to_string()),
                            source: Some({
                                let source = mount
                                    .source
                                    .canonicalize()
                                    .context("failed to canonicalize path")?;
                                source.display().to_string()
                            }),
                            typ: Some(MountTypeEnum::BIND),
                            read_only: Some(mount.read_only),
                            ..Default::default()
                        })
                    })
                    .collect::<anyhow::Result<Vec<_>>>()?,
            ),
            memory: Some(stage.memory_limit.as_u64() as _),
            memory_swap: Some(stage.swap_limit.as_u64() as _),
            cpuset_cpus: stage.cpu_cores.clone(),
            nano_cpus: Some((stage.cpu_limit * 1e9) as _),
            restart_policy: Some(RestartPolicy {
                name: Some(RestartPolicyNameEnum::NO),
                maximum_retry_count: Some(0),
                ..Default::default()
            }),
            cap_drop: Some(vec!["ALL".to_string()]),
            cap_add: Some(vec!["DAC_OVERRIDE".to_string()]),
            security_opt: Some(vec!["no-new-privileges".to_string()]),
            pids_limit: Some(512),
            network_mode: Some(
                if stage.networking {
                    "typst-internal"
                } else {
                    "none"
                }
                .to_string(),
            ),
            ..Default::default()
        }),
        ..Default::default()
    };

    let name = format!("typster-{}", stage.image.replace('/', "-"));
    let options = CreateContainerOptions {
        name: &name as &str,
        platform: Some("linux/amd64"),
    };

    let container = docker
        .create_container::<&str, &str>(Some(options), secure_config)
        .await
        .with_context(|| format!("failed to create containerm {:?}", stage))?;

    for warning in container.warnings {
        tracing::warn!("{}", warning.trim());
    }

    docker
        .start_container::<&str>(&container.id, None)
        .await
        .context("failed to start container")?;

    Ok(Container {
        docker: docker.clone(),
        timeout: stage.hard_timeout.into(),
        id: container.id,
        stopped: false,
    })
}
