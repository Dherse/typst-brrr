use std::{collections::HashMap, path::Path};

use anyhow::Context;
use bollard::{
    container::{Config, CreateContainerOptions},
    image::BuildImageOptions,
    service::HostConfig,
    Docker,
};
use futures_util::StreamExt;

const CLONE_TAR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/clone.tar"));
const FETCH_TAR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/fetch.tar"));
const BUILD_TAR: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/build.tar"));
const BENCH_END_TO_END: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/bench-end-to-end.tar"));

const DOCKER_IMAGES: &[(&str, &str, &[u8])] = &[
    ("clone", "typst-clone", CLONE_TAR),
    ("fetch", "typst-fetch", FETCH_TAR),
    ("build", "typst-build", BUILD_TAR),
    (
        "bench-end-to-end",
        "typst-bench-end-to-end",
        BENCH_END_TO_END,
    ),
];

pub struct DockerImages<'a>(&'a Docker, HashMap<String, String>);

impl<'a> DockerImages<'a> {
    pub fn docker(&self) -> &'a Docker {
        self.0
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.1.get(key).map(|s| s.as_str())
    }

    pub fn get_clone(&self) -> Option<&str> {
        self.get("clone")
    }

    pub fn get_fetch(&self) -> Option<&str> {
        self.get("fetch")
    }

    pub fn get_build(&self) -> Option<&str> {
        self.get("build")
    }

    pub fn get_bench_end_to_end(&self) -> Option<&str> {
        self.get("bench-end-to-end")
    }

    pub async fn run_clone(
        &self,
        repo: &str,
        commit: &str,
        git_dir: &Path,
    ) -> anyhow::Result<()> {
        let image = self.get_clone().context("clone image not found")?;

        let host_config = HostConfig {
            mounts: Some(vec![bollard::models::Mount {
                target: Some("/data".to_string()),
                source: Some(git_dir.canonicalize()?.to_str().context("invalid git dir")?.to_string()),
                typ: Some(bollard::service::MountTypeEnum::BIND),
                ..Default::default()
            }]),
            ..Default::default()
        };

        let config = Config {
            image: Some(image.to_string()),
            env: Some(vec![
                format!("REPO_URL={}", repo),
                format!("COMMIT={}", commit),
            ]),
            host_config: Some(host_config),
            ..Default::default()
        };

        self
            .docker()
            .create_container(
                Some(CreateContainerOptions {
                    name: "clone-container",
                    platform: None,
                }),
                config,
            )
            .await
            .context("failed to create container")?;

        self.docker()
            .start_container::<String>("clone-container", None)
            .await
            .context("failed to start container")?;

        let mut stream = self.docker().wait_container::<String>("clone-container", None);

        while let Some(result) = stream.next().await {
            let _result = result;
        }

        self.docker().remove_container("clone-container", None).await?;

        Ok(())
    }

    pub async fn run_fetch(
        &self,
        git_dir: &Path,
        cargo_dir: &Path,
    ) -> anyhow::Result<()> {
        let image = self.get_fetch().context("fetch image not found")?;

        let host_config = HostConfig {
            mounts: Some(vec![
                bollard::models::Mount {
                    target: Some("/data".to_string()),
                    source: Some(git_dir.canonicalize()?.to_str().context("invalid git dir")?.to_string()),
                    typ: Some(bollard::service::MountTypeEnum::BIND),
                    ..Default::default()
                },
                bollard::models::Mount {
                    target: Some("/cargo".to_string()),
                    source: Some(cargo_dir.canonicalize()?.to_str().context("invalid cargo dir")?.to_string()),
                    typ: Some(bollard::service::MountTypeEnum::BIND),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        };

        let config = Config {
            image: Some(image.to_string()),
            host_config: Some(host_config),
            ..Default::default()
        };

        self
            .docker()
            .create_container(
                Some(CreateContainerOptions {
                    name: "fetch-container",
                    platform: None,
                }),
                config,
            )
            .await
            .context("failed to create container")?;

        self.docker()
            .start_container::<String>("fetch-container", None)
            .await
            .context("failed to start container")?;

        let mut stream = self.docker().wait_container::<String>("fetch-container", None);

        while let Some(result) = stream.next().await {
            let _result = result;
        }

        self.docker().remove_container("fetch-container", None).await?;

        Ok(())
    }

    pub async fn run_build(
        &self,
        git_dir: &Path,
        cargo_dir: &Path,
    ) -> anyhow::Result<()> {
        let image = self.get_build().context("build image not found")?;

        let host_config = HostConfig {
            mounts: Some(vec![
                bollard::models::Mount {
                    target: Some("/data".to_string()),
                    source: Some(git_dir.canonicalize()?.to_str().context("invalid git dir")?.to_string()),
                    typ: Some(bollard::service::MountTypeEnum::BIND),
                    ..Default::default()
                },
                bollard::models::Mount {
                    target: Some("/cargo".to_string()),
                    source: Some(cargo_dir.canonicalize()?.to_str().context("invalid cargo dir")?.to_string()),
                    typ: Some(bollard::service::MountTypeEnum::BIND),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        };

        let config = Config {
            image: Some(image.to_string()),
            host_config: Some(host_config),
            ..Default::default()
        };

        self
            .docker()
            .create_container(
                Some(CreateContainerOptions {
                    name: "build-container",
                    platform: None,
                }),
                config,
            )
            .await
            .context("failed to create container")?;

        self.docker()
            .start_container::<String>("build-container", None)
            .await
            .context("failed to start container")?;

        let mut stream = self.docker().wait_container::<String>("build-container", None);

        while let Some(result) = stream.next().await {
            let _result = result;
        }

        self.docker().remove_container("build-container", None).await?;

        Ok(())
    }

    pub async fn run_e2e_benches(
        &self,
        git_dir: &Path,
        cargo_dir: &Path,
        sample_dir: &Path,
        data_dir: &Path,
        test_files: &[&str],
    ) -> anyhow::Result<()> {
        let image = self.get_bench_end_to_end().context("bench-end-to-end image not found")?;

        let bin_dir = git_dir.join("target").join("release");

        let host_config = HostConfig {
            mounts: Some(vec![
                bollard::models::Mount {
                    target: Some("/binary".to_string()),
                    source: Some(bin_dir.canonicalize()?.to_str().context("invalid git dir")?.to_string()),
                    typ: Some(bollard::service::MountTypeEnum::BIND),
                    ..Default::default()
                },
                bollard::models::Mount {
                    target: Some("/cargo".to_string()),
                    source: Some(cargo_dir.canonicalize()?.to_str().context("invalid cargo dir")?.to_string()),
                    typ: Some(bollard::service::MountTypeEnum::BIND),
                    ..Default::default()
                },
                bollard::models::Mount {
                    target: Some("/samples".to_string()),
                    source: Some(sample_dir.canonicalize()?.to_str().context("invalid cargo dir")?.to_string()),
                    typ: Some(bollard::service::MountTypeEnum::BIND),
                    ..Default::default()
                },
                bollard::models::Mount {
                    target: Some("/data".to_string()),
                    source: Some(data_dir.canonicalize()?.to_str().context("invalid cargo dir")?.to_string()),
                    typ: Some(bollard::service::MountTypeEnum::BIND),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        };

        let config = Config {
            image: Some(image.to_string()),
            host_config: Some(host_config),
            env: Some(vec![
                "FILE_LIST=".to_string() + &test_files.join(","),
                "WARMUPS=5".to_string(),
                "RUNS=200".to_string(),
            ]),
            ..Default::default()
        };

        self
            .docker()
            .create_container(
                Some(CreateContainerOptions {
                    name: "bench-container",
                    platform: None,
                }),
                config,
            )
            .await
            .context("failed to create container")?;

        self.docker()
            .start_container::<String>("bench-container", None)
            .await
            .context("failed to start container")?;

        let mut stream = self.docker().wait_container::<String>("bench-container", None);

        while let Some(result) = stream.next().await {
            let _result = result;
        }

        self.docker().remove_container("bench-container", None).await?;

        Ok(())
    }
}

pub async fn build_images<'a>(docker: &'a Docker) -> anyhow::Result<DockerImages<'a>> {
    let mut out = HashMap::default();
    for (key, name, tar) in DOCKER_IMAGES {
        let build_options = BuildImageOptions {
            dockerfile: "dockerfile",
            t: name,
            ..Default::default()
        };

        let mut stream = docker.build_image(build_options, None, Some(tar.to_vec().into()));
        while let Some(image) = stream.next().await {
            image.context("failed to build docker image")?;
        }

        out.insert(key.to_string(), name.to_string());
    }

    Ok(DockerImages(docker, out))
}
