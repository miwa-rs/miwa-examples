use axum_basic::{todo_api_extension, todo_repo_extension};
use miwa::core::Miwa;
use miwa_axum::extensions::web_service_extension;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    Miwa::prepare()
        .with_env("MIWA")
        .build()?
        .add_extension(web_service_extension)
        .add_extension(todo_api_extension)
        .add_extension(todo_repo_extension)
        .start()
        .await?;

    Ok(())
}
