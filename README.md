# Rust Backend Stack

## Setup

### Installing Rust and Cargo

Install Rust as described [here](https://doc.rust-lang.org/book/ch01-01-installation.html).

### Installing `sqlx-cli`

SQLx is an async, pure Rust SQL crate featuring compile-time checked queries without a DSL.

SQLx-CLI is SQLx's associated command-line utility for managing databases, migrations, and enabling "offline" mode with sqlx::query!() and friends.
It is published on the Cargo crates registry as `sqlx-cli` and can be installed like so:

```shell
cargo install sqlx-cli --features postgres
```

### Running Postgres

The following script will start the latest version of Postgres using [Docker], create the database and run the migrations.

```shell
./scripts/init_db.sh
```

### Preparing SQLx data

There are 3 steps to building with "offline mode":

- Enable the SQLx's Cargo feature offline
  - E.g. in your Cargo.toml, sqlx = { features = [ "offline", ... ] }
- Save query metadata for offline usage
  - `cargo sqlx prepare`
- Build

### Configuring the Application

Configuring the backend application is done, preferentially, via environment variables.
To make working with environment variables easier during development, we can use [.env files] to avoid having
to define the variables every time.

As a starting point, you can simply `cp sample.env .env` in this repo and modify the `.env` file as described by
the comments there.

### Starting the Application

With everything else set up, all you need to do at this point is:

```shell
cargo run
```

If successful, the API server is now listening at port 8080.

#### Hot Reload

Use [`cargo-watch`](https://crates.io/crates/cargo-watch) for hot reloading the server.

```bash
cargo watch -x run
```

## License

Licensed under MIT license. Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, shall be licensed as above, without any additional terms or conditions.

[Docker]: https://www.docker.com/
[.env files]: https://github.com/dotenv-rs/dotenv
