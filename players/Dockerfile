FROM docker.io/lukemathwalker/cargo-chef:latest-rust-1.58.0 AS chef
COPY common /common
WORKDIR /app

FROM chef AS planner
COPY players/ .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY players/ .
RUN cargo build --release

# Runtime
FROM docker.io/library/debian:buster-slim AS runtime
WORKDIR /app
COPY --from=build /app/target/release/players /usr/local/bin/
EXPOSE 8003
ENTRYPOINT ["/usr/local/bin/players"]
