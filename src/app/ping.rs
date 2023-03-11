use crate::app::ApiContext;
use axum::body::Body;
use axum::routing::get;
use axum::Router;

pub fn router() -> Router<ApiContext, Body> {
    Router::new().route("/api/ping", get(ping))
}

async fn ping() -> &'static str {
    "pong"
}
