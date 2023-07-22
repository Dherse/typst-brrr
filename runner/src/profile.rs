use std::path::{Path, PathBuf};

use anyhow::Context;
use bytesize::ByteSize;
use duration_string::DurationString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// Whether to delete the sandbox directory on exit
    pub delete_on_exit: bool,

    /// The profiles to use for
    pub profiles: Profiles,

    /// The stages to use for the sandbox
    pub stages: Stages,

    /// The sample configuration
    pub samples: Samples,

    /// The directory to clone the repository into
    pub workdir: PathBuf,
}

impl Profile {
    pub async fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let path = path.as_ref();
        let file = tokio::fs::read_to_string(path)
            .await
            .context("failed to read profile file")?;

        Ok(toml::de::from_str(&file).context("failed to parse profile file")?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Samples {
    /// The directory that contains the samples
    pub root: PathBuf,

    /// The sample files, relative to the root directory
    pub files: Vec<PathBuf>,

    /// The training files for PGO, relative to the root directory
    pub training: Vec<PathBuf>,
}

impl Samples {
    pub fn to_env(&self) -> Option<String> {
        let path = PathBuf::from("/samples");

        self.files
            .iter()
            .map(|p| path.join(p))
            .map(|path| path.display().to_string())
            .reduce(|a, b| format!("{},{}", a, b))
    }

    pub fn to_training_env(&self) -> Option<String> {
        let path = PathBuf::from("/samples");

        self.training
            .iter()
            .map(|p| path.join(p))
            .map(|path| path.display().to_string())
            .reduce(|a, b| format!("{},{}", a, b))
    }

    pub fn to_results_file(&self, results: impl AsRef<Path>) -> Vec<PathBuf> {
        self.files
            .iter()
            .map(|p| p.file_name().expect("missing file name"))
            .map(|p| results.as_ref().join(p))
            .map(|p| p.with_extension("json"))
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profiles {
    pub main: ProfileSettings,

    pub other: ProfileSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stages {
    pub clone: Stage,
    pub fetch: Stage,
    pub build: Stage,
    pub bench_e2e: Stage,
    pub bench_walltime: Stage,
    pub pgo_build: Stage,
    pub pgo_build_profile: Stage,
    pub pgo_profile: Stage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileSettings {
    pub warmups: u32,
    pub runs: u32,
    pub interval: DurationString,
    pub work: u32,
    pub sleep: DurationString,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stage {
    pub image: String,
    pub soft_timeout: DurationString,
    pub hard_timeout: DurationString,
    pub memory_limit: ByteSize,
    pub swap_limit: ByteSize,
    pub cpu_limit: f64,
    pub networking: bool,
    pub cpu_cores: Option<String>,
}
