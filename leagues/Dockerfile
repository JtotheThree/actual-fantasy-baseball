FROM docker.io/lukemathwalker/cargo-chef:latest-rust-1.58.0 AS chef
COPY common /common
WORKDIR /app

FROM chef AS planner
COPY leagues/ .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY leagues/ .
RUN cargo build --release

# Runtime
FROM docker.io/library/debian:buster-slim AS runtime
WORKDIR /app
COPY --from=build /app/target/release/leagues /usr/local/bin/
EXPOSE 8001
ENTRYPOINT ["/usr/local/bin/leagues"]


# Install cargo-chef
# FROM docker.io/library/rust:1.58 AS chef
# RUN cargo install cargo-chef
# RUN mkdir /usr/src/build/
# COPY common /usr/src/common
# WORKDIR /usr/src/build/

# # Prepare cargo-chef recipe
# FROM chef AS planner
# # TODO: Remove the cargo workspace unfortunately, changing any microservices Cargo.toml causes Cargo.lock to change and rebuild EVERYTHING
# COPY Cargo.lock .
# COPY leagues/ .
# RUN cargo chef prepare --recipe-path recipe.json

# FROM chef AS build
# COPY --from=planner /usr/src/build/recipe.json recipe.json
# # Build dependencies
# RUN cargo chef cook --release --recipe-path recipe.json
# # Build application
# COPY common /usr/src/common
# COPY Cargo.lock .
# COPY leagues/ .
# RUN cargo build --release

# # Runtime
# FROM docker.io/library/debian:buster-slim AS runtime
# WORKDIR /app
# COPY --from=build /usr/src/build/target/release/leagues /usr/local/bin/
# EXPOSE 8001
# ENTRYPOINT ["/usr/local/bin/leagues"]
