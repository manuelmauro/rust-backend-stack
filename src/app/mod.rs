use crate::config::Config;
use anyhow::Context;
use axum::{extract::FromRef, Router};
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

/// Defines a common error type to use for all request handlers.
mod error;

/// Contains definitions for application-specific parameters to handler functions.
mod extractor;

/// A catch-all module for other common types in the API.
mod types;

// Modules introducing API routes.
pub mod ping;
pub mod users;

pub use error::{Error, ResultExt};

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// The core type through which handler functions can access common API state.
///
/// This can be accessed by adding a parameter `State<ApiContext>` to a handler function's
/// parameters.
#[derive(Clone, FromRef)]
pub struct ApiContext {
    config: Arc<Config>,
    db: PgPool,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        ping::ping,
        users::create_user,
        users::login_user,
        users::get_current_user,
        users::update_user
    ),
    components(schemas(
        users::schema::NewUserBody,
        users::schema::NewUser,
        users::schema::LoginUserBody,
        users::schema::LoginUser,
        users::schema::UpdateUserBody,
        users::schema::UpdateUser,
        users::schema::UserResponse,
        users::schema::User,
    )),
    modifiers(&SecurityAddon),
)]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}

pub async fn serve(config: Config, db: PgPool) -> anyhow::Result<()> {
    let app = api_router()
        .with_state(ApiContext {
            config: Arc::new(config),
            db,
        })
        .layer(TraceLayer::new_for_http());

    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(app.into_make_service())
        .await
        .context("error running HTTP server")
}

fn api_router() -> Router<ApiContext> {
    users::router()
        .merge(ping::router())
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/swagger.json", ApiDoc::openapi()))
}
