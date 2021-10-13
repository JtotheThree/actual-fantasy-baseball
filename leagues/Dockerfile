FROM rust:1.55 as build

ENV CARGO_TERM_COLOR always
RUN apt-get update && apt-get install -y libpq-dev clang

# create empty project for caching dependencies
RUN USER=root cargo new --bin /build-dir/docker-build
COPY /common/ /build-dir/common/
WORKDIR /build-dir/docker-build
COPY /Cargo.lock ./
COPY /leagues/Cargo.toml ./
# cache dependencies
RUN cargo install --path . --locked

COPY leagues/ .
RUN cargo install --path .

FROM debian:buster-slim as production
RUN apt-get update && apt-get install -y libpq-dev curl
COPY --from=build /usr/local/cargo/bin/leagues /usr/local/bin/leagues
CMD ["leagues"]
EXPOSE 8001