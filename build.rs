use std::{
    env,
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::Context;

pub fn main() -> anyhow::Result<()> {
    let docker_path = "./docker-images/";
    let build_dir = PathBuf::from(env::var("OUT_DIR").context("OUT_DIR not set")?);

    build_one(docker_path, &build_dir, "clone")?;
    build_one(docker_path, &build_dir, "fetch")?;
    build_one(docker_path, &build_dir, "build")?;
    build_one(docker_path, &build_dir, "bench-end-to-end")?;

    Ok(())
}

fn build_one<P: AsRef<Path>>(docker_path: P, build_dir: &Path, source: &str) -> anyhow::Result<()> {
    let path = build_dir.join(format!("{source}.tar"));
    let in_path = docker_path.as_ref().join(format!("{source}/"));
    println!("cargo:rerun-if-changed={}", in_path.display());

    let file = File::create(path).context("failed to create TAR file")?;
    let mut archive = tar::Builder::new(file);

    archive
        .append_dir_all("./", in_path)
        .context("failed to append directory to TAR file")?;

    archive.finish().context("failed to finish TAR file")?;

    Ok(())
}
