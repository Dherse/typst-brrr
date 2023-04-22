use tracing_subscriber::{fmt::SubscriberBuilder, EnvFilter};

pub mod config;
pub mod sandbox;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    SubscriberBuilder::default()
        .with_env_filter(EnvFilter::from_default_env())
        .without_time()
        .init();

    let sandbox = sandbox::Sandbox::new(
        "https://github.com/Dherse/typst",
        "content-rework"
    )?;

    dbg!(sandbox.clone().await?);
    dbg!(sandbox.fetch().await?);

    Ok(())
}
