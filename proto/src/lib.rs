use serde::{Deserialize, Serialize};

pub use bincode::{deserialize_from, serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BenchQuery {
    pub id: String,
    pub repo: String,
    pub commit: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct StageOutput {
    pub stdout: Vec<String>,
    pub stderr: Vec<String>,
    pub exitcode: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BenchSamples {
    pub name: String,
    pub user_time: Vec<f64>,
    pub system_time: Vec<f64>,
    pub virtual_memory: Vec<f64>,
    pub resident_memory: Vec<f64>,
    pub cpu_percent: Vec<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BenchWalltimeSamples {
    pub name: String,
    pub walltime: Vec<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BenchResult {
    Success {
        id: String,
        samples: Vec<BenchSamples>,
        walltimes: Vec<BenchWalltimeSamples>,
        clone: StageOutput,
        fetch: StageOutput,
        build: StageOutput,
        bench_e2e: StageOutput,
        bench_walltime: StageOutput,
    },
    Failure {
        id: String,
        stage: String,
        clone: Option<StageOutput>,
        fetch: Option<StageOutput>,
        build: Option<StageOutput>,
        bench_e2e: Option<StageOutput>,
        bench_walltime: Option<StageOutput>,
    },
}
