use anyhow::Context;
use clap::Parser;
use sqlx::postgres::PgPoolOptions;

use rust_backend_stack::app;
use rust_backend_stack::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let config = Config::parse();

    let db = PgPoolOptions::new()
        .max_connections(50)
        .connect(&config.database_url)
        .await
        .context("could not connect to database_url")?;

    // This embeds database migrations in the application binary so we can ensure the database
    // is migrated correctly on startup
    sqlx::migrate!().run(&db).await?;

    app::serve(config, db).await?;

    Ok(())
}
