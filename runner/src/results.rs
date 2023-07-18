use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BenchmarkResults {
    /// The command that was executed.
    pub command: String,

    /// The metric that was measured.
    pub metric: Metric,

    /// The samples that were taken, see [`Metric`] for the meaning of the
    /// values.
    pub samples: Vec<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct SamplingResults {
    /// The configuration used for sampling.
    pub sampling_config: SampleConfig,

    /// The results of the sampling.
    pub samples: Vec<BenchmarkResults>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Metric {
    /// The time, in nano-seconds, that the command took to execute.
    Time,

    /// The peak virtual memory usage, in bytes, of the command.
    VirtualMemory,

    /// The peak resident memory usage, in bytes, of the command.
    ResidentMemory,

    /// The peak CPU load, in percentage, of the command.
    Load,

    /// The user CPU time, in nano-seconds, of the command.
    UserCpuTime,

    /// The system CPU time, in nano-seconds, of the command.
    SystemCpuTime,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SampleConfig {
    pub n_warmup: usize,
    pub n_samples: usize,
    pub sleep_time: Duration,
    pub silly_work: Option<usize>,
}
