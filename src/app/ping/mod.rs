use crate::app::ApiContext;
use axum::body::Body;
use axum::routing::get;
use axum::Router;

pub fn router() -> Router<ApiContext, Body> {
    Router::new().route("/api/ping", get(ping))
}

#[utoipa::path(
    get,
    path = "/api/ping",
    responses(
        (status = 200, description = "Pong.")
    )
)]
async fn ping() -> &'static str {
    "pong"
}
