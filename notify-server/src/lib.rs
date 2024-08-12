mod sse;

use crate::sse::sse_handler;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Router;

pub fn get_router() -> Router {
    Router::new()
        .route("/events", get(sse_handler))
        .route("/index", get(index_handler))
}

async fn index_handler() -> impl IntoResponse {
    let html = include_str!("../index.html");
    Html(html)
}
