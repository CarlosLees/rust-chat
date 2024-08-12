use crate::config::AppConfig;
use crate::handlers::*;
use axum::routing::{get, patch, post};
use axum::Router;
use std::ops::Deref;
use std::sync::Arc;
use tracing::info;

#[derive(Debug, Clone)]
pub(crate) struct AppState {
    pub(crate) inner: Arc<AppStateInner>,
}

#[derive(Debug)]
pub(crate) struct AppStateInner {
    pub(crate) config: AppConfig,
}

// 调用state.config => state.inner.config
impl Deref for AppState {
    type Target = AppStateInner;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl AppState {
    pub fn new(config: AppConfig) -> Self {
        Self {
            inner: Arc::new(AppStateInner { config }),
        }
    }
}

pub fn get_router(config: AppConfig) -> Router {
    let state = AppState::new(config);
    info!("{}", state.config.server.port);

    let api = Router::new()
        .route("/signIn", post(sign_in_handler))
        .route("/signUp", post(sign_up_handler))
        .route("/chat", get(list_chat_handler).post(create_chat_handler))
        .route(
            "/chat/:id",
            patch(update_chat_handler)
                .delete(delete_chat_handler)
                .post(send_message_handler),
        )
        .route("/chat/:id/messages", get(list_message_handler));

    Router::new()
        .route("/", get(index_handler))
        .nest("/api", api)
        .with_state(state)
}
