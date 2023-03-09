# Rust as the base image
FROM rust:1.67 as build

# Create a new empty shell project
RUN USER=root cargo new --bin rust-backend-stack
WORKDIR /rust-backend-stack

# Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./sqlx-data.json ./sqlx-data.json
COPY ./migrations ./migrations

# Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY ./src ./src

# Build for release.
RUN rm ./target/release/deps/rust_backend_stack*
RUN cargo build --release

# TODO use a smaller image
# The final base image
FROM rust:1.67

# Copy from the previous build
COPY --from=build /rust-backend-stack/target/release/rust-backend-stack /usr/src/rust-backend-stack

# Run the binary
CMD ["/usr/src/rust-backend-stack"]
