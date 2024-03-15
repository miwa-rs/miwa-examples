use axum::routing::{get, patch};
use axum::Router;
use miwa::core::{Extension, ExtensionConfig, MiwaContext, MiwaResult};
use miwa::derive::{extension, ExtensionConfig};
use miwa_axum::{WebService, WebServiceConfigBuilder};
use serde::Deserialize;

use crate::handler::{todos_create, todos_delete, todos_get, todos_index, todos_update};
use crate::repo::{InMemoryTodoRepo, TodoRepoImpl};

pub struct TodoApiExtension;

#[async_trait::async_trait]
impl Extension for TodoApiExtension {
    async fn start(&self) -> MiwaResult<()> {
        Ok(())
    }

    async fn shutdown(&self) -> MiwaResult<()> {
        Ok(())
    }
}

pub struct TodoStoreExtension;

#[async_trait::async_trait]
impl Extension for TodoStoreExtension {
    async fn start(&self) -> MiwaResult<()> {
        Ok(())
    }

    async fn shutdown(&self) -> MiwaResult<()> {
        Ok(())
    }
}

#[extension(name = "Todo In Memory extension", provides(TodoRepoImpl))]
pub async fn todo_repo_extension(ctx: &MiwaContext) -> MiwaResult<TodoStoreExtension> {
    ctx.register(TodoRepoImpl::new(InMemoryTodoRepo::new()));
    Ok(TodoStoreExtension)
}

#[derive(Deserialize, ExtensionConfig)]
#[config(prefix = "web.todo")]
pub struct TodoApi {
    port: u16,
}

#[extension(name = "Todo API extension")]
pub async fn todo_api_extension(
    web: WebService,
    repo: TodoRepoImpl,
    ExtensionConfig(cfg): ExtensionConfig<TodoApi>,
) -> MiwaResult<TodoApiExtension> {
    web.add_server(
        WebServiceConfigBuilder::default()
            .port(cfg.port)
            .build()
            .unwrap(),
    );
    configure_web_server(web, repo);
    Ok(TodoApiExtension)
}

fn configure_web_server(web: WebService, repo: TodoRepoImpl) {
    web.merging(
        "default",
        Router::new()
            .route("/todos", get(todos_index).post(todos_create))
            .route(
                "/todos/:id",
                patch(todos_update).delete(todos_delete).get(todos_get),
            )
            .with_state(repo),
    );
}
