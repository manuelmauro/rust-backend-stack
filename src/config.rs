/// The configuration parameters for the application.
#[derive(clap::Parser)]
pub struct Config {
    /// The connection URL for the Postgres database.
    #[clap(long, env)]
    pub database_url: String,

    /// The HMAC signing and verification key used for login tokens (JWTs).
    #[clap(long, env)]
    pub hmac_key: String,
}
