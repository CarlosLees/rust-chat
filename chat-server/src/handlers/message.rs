use axum::extract::Path;
use axum::response::IntoResponse;

pub(crate) async fn send_message_handler(Path(_id): Path<u16>) -> impl IntoResponse {
    "send message"
}

pub(crate) async fn list_message_handler(Path(_id): Path<u16>) -> impl IntoResponse {
    "list message"
}
