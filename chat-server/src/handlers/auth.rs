use axum::response::IntoResponse;

pub(crate) async fn sign_in_handler() -> impl IntoResponse {
    "sign in"
}

pub(crate) async fn sign_up_handler() -> impl IntoResponse {
    "sign up"
}
