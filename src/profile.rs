use std::{collections::HashMap, path::PathBuf, time::Duration};

use anyhow::Context;
use duration_string::DurationString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Profile {
    pub profiles: Profiles,

    pub delete_on_exit: bool,

    pub stages: HashMap<String, Stage>,

    pub samples: HashMap<String, PathBuf>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Profiles {
    pub main: ProfileSettings,

    pub other: ProfileSettings,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileSettings {
    pub samples: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stage {
    pub soft_timeout: DurationString,
    pub hard_timeout: DurationString,
    pub memory_limit: String,
    pub swap_limit: String,
    pub cpu_limit: f64,
    pub networking: bool,
}

impl Default for Stage {
    fn default() -> Self {
        Self {
            soft_timeout: DurationString::new(Duration::from_secs(120)),
            hard_timeout: DurationString::new(Duration::from_secs(180)),
            memory_limit: "512m".to_string(),
            swap_limit: "768m".to_string(),
            cpu_limit: 1.0,
            networking: false,
        }
    }
}
