use miwa::core::{Extension, System, SystemContext, SystemResult};
use miwa::derive::{extension, Injectable};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    System::prepare()?.start().await?;

    Ok(())
}
