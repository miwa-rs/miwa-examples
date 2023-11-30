use miwa::core::{Extension, System, SystemContext, SystemResult};
use miwa::derive::{extension, Injectable};
use miwa_axum::extensions::web_service_extension;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    System::prepare()?
        .add_extension(web_service_extension)
        .start()
        .await?;

    Ok(())
}
