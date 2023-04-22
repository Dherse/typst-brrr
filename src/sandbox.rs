use std::{
    os::unix::prelude::PermissionsExt,
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::{bail, Context};
use rand::Rng;
use tokio::process::Command;

pub struct Sandbox {
    id: String,
    git: PathBuf,
    cargo: PathBuf,
    results: PathBuf,

    repository: String,
    commit: String,
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        if let Err(e) = std::fs::remove_dir_all(&self.git) {
            tracing::error!("failed to remove git directory: {}", e);
        }

        if let Err(e) = std::fs::remove_dir_all(&self.cargo) {
            tracing::error!("failed to remove cargo directory: {}", e);
        }

        if let Err(e) = std::fs::remove_dir_all(&self.results) {
            tracing::error!("failed to remove results directory: {}", e);
        }
    }
}

impl Sandbox {
    pub async fn new<P: AsRef<Path>, S1: ToString, S2: ToString>(
        root: P,
        repository: S1,
        commit: S2,
    ) -> anyhow::Result<Self> {
        //generate random ID of length 10 using the rand crate
        let id = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(10)
            .map(char::from)
            .collect::<String>();

        let git = root
            .as_ref()
            .join(format!("{id}-git"));

        tokio::fs::create_dir_all(&git)
            .await
            .context("failed to create git directory")?;

        let cargo = root
            .as_ref()
            .join(format!("{id}-cargo"));

        tokio::fs::create_dir_all(&cargo)
            .await
            .context("failed to create cargo directory")?;

        let results = root
            .as_ref()
            .join(format!("{id}-results"));

        tokio::fs::create_dir_all(&results)
            .await
            .context("failed to create results directory")?;

        Ok(Self {
            id,
            git,
            cargo,
            results,
            repository: repository.to_string(),
            commit: commit.to_string(),
        })
    }

    pub async fn clone(&self) -> anyhow::Result<Output> {
        let mut cmd = basic_secure_docker_command(Duration::from_secs(10), true, 1.0);

        cmd.arg("--env");
        cmd.arg(format!("REPO_URL={}", self.repository));
        cmd.arg("--env");
        cmd.arg(format!("COMMIT={}", self.commit));

        cmd.arg("--mount");
        cmd.arg(format!(
            "type=bind,source={},target=/typster",
            self.git.display()
        ));

        cmd.arg("typst/clone");

        let out = run_command_with_timout(cmd, Duration::from_secs(30)).await?;

        match out {
            Some(out) => Ok(out),
            None => bail!("failed to clone repository"),
        }
    }

    pub async fn fetch(&self) -> anyhow::Result<Output> {
        let mut cmd = basic_secure_docker_command(Duration::from_secs(60), true, 1.0);

        cmd.arg("--mount");
        cmd.arg(format!(
            "type=bind,source={},target=/typster,readonly",
            self.git.display()
        ));

        cmd.arg("--mount");
        cmd.arg(format!(
            "type=bind,source={},target=/cargo",
            self.cargo.display()
        ));

        cmd.arg("typst/fetch");

        let out = run_command_with_timout(cmd, Duration::from_secs(120)).await?;

        match out {
            Some(out) => Ok(out),
            None => bail!("failed to fetch crates"),
        }
    }

    pub async fn build(&self) -> anyhow::Result<Output> {
        let mut cmd = basic_secure_docker_command(Duration::from_secs(1200), true, 4.0);

        cmd.arg("--mount");
        cmd.arg(format!(
            "type=bind,source={},target=/typster",
            self.git.display()
        ));

        cmd.arg("--mount");
        cmd.arg(format!(
            "type=bind,source={},target=/cargo,readonly",
            self.cargo.display()
        ));

        cmd.arg("typst/build");

        let out = run_command_with_timout(cmd, Duration::from_secs(1200)).await?;

        match out {
            Some(out) => Ok(out),
            None => bail!("failed to build project"),
        }
    }
}

pub struct CloneOutput {
    stdout: Vec<String>,
    stderr: Vec<String>,
}

// We must create a world-writable files (rustfmt) and directories
// (LLVM IR) so that the process inside the Docker container can write
// into it.
//
// This problem does *not* occur when using the indirection of
// docker-machine.
fn wide_open_permissions() -> std::fs::Permissions {
    PermissionsExt::from_mode(0o777)
}

macro_rules! docker_command {
    ($($arg:expr),* $(,)?) => ({
        let mut cmd = Command::new("docker");
        $( cmd.arg($arg); )*
        cmd.kill_on_drop(true);
        cmd
    });
}

#[derive(Debug)]
pub struct Output {
    pub cmd: String,
    pub exitcode: i32,
    pub stdout: Vec<String>,
    pub stderr: Vec<String>,
}

async fn run_command_with_timout(
    mut cmd: Command,
    timeout: Duration,
) -> anyhow::Result<Option<Output>> {
    let output = tokio::time::timeout(timeout, cmd.output())
        .await
        .context("docker command timed out")?
        .context("command failed")?;

    // if we fail to run the container, we return None
    if !output.status.success() {
        for line in String::from_utf8_lossy(&output.stderr).lines() {
            tracing::error!("{}", line);
        }

        return Ok(None);
    }

    let output = String::from_utf8_lossy(&output.stdout);
    let id = output
        .lines()
        .next()
        .context("missing container id")?
        .trim();

    let mut wait_cmd = docker_command!("wait", id);
    let wait_output = tokio::time::timeout(timeout, wait_cmd.output())
        .await
        .context("docker command timed out")?
        .context("command failed")?;

    let status_code = String::from_utf8_lossy(&wait_output.stdout)
        .lines()
        .next()
        .unwrap_or("")
        .trim()
        .parse()
        .unwrap_or(i32::MAX);

    let mut logs_cmd = docker_command!("logs", id);
    let logs_output = tokio::time::timeout(timeout, logs_cmd.output())
        .await
        .context("docker command timed out")?
        .context("command failed")?;

    let stdout = String::from_utf8_lossy(&logs_output.stdout)
        .lines()
        .map(str::trim)
        .map(ToString::to_string)
        .collect::<Vec<String>>();

    let stderr = String::from_utf8_lossy(&logs_output.stderr)
        .lines()
        .map(str::trim)
        .map(ToString::to_string)
        .collect::<Vec<String>>();

    let mut rm_cmd = docker_command!("rm", "--force", id);
    rm_cmd
        .status()
        .await
        .context("failed to remove container")?;

    Ok(Some(Output {
        cmd: format!("{:?}", cmd.as_std()),
        exitcode: status_code,
        stdout,
        stderr,
    }))
}

fn basic_secure_docker_command(timeout: Duration, allow_network: bool, max_cores: f32) -> Command {
    let mut cmd = docker_command!(
        "run",
        "--platform",
        "linux/amd64",
        "--detach",
        "--cap-drop=ALL",
        // Needed to allow overwriting the file
        "--cap-add=DAC_OVERRIDE",
        "--security-opt=no-new-privileges",
        "--workdir",
        "/typster",
        "--memory",
        "512m",
        "--memory-swap",
        "640m",
        "--cpus",
        format!("{max_cores:.1}"),
        "--env",
        format!("TYPSTER_TIMEOUT={}", timeout.as_secs()),
    );

    if !allow_network {
        cmd.arg("--network");
        cmd.arg("none");
    } else {
        cmd.arg("--network");
        cmd.arg("typst-internal");

        cmd.arg("--env");
        cmd.arg("HTTP_PROXY=http://172.19.0.2:3128");

        cmd.arg("--env");
        cmd.arg("HTTPS_PROXY=http://172.19.0.2:3128");

        cmd.arg("--env");
        cmd.arg("FTP_PROXY=http://172.19.0.2:3128");
    }

    if cfg!(feature = "fork-bomb-prevention") {
        cmd.args(&["--pids-limit", "512"]);
    }

    cmd.kill_on_drop(true);

    cmd
}
