FROM rust:1.78-slim-buster as build

# create a new empty shell project
RUN USER=root cargo new --bin metools_backend
WORKDIR /metools_backend
RUN apt-get update && apt-get install -y pkg-config libssl-dev clang curl libcrypto++-dev libpq-dev
# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/metools_backend*
RUN cargo build --release

# our final base
FROM debian:buster-slim
RUN apt-get update && apt-get install -y libssl-dev ca-certificates libcrypto++-dev libpq-dev curl && update-ca-certificates
# copy the build artifact from the build stage
WORKDIR /app
COPY --from=build /metools_backend/target/release/metools_backend .
RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/download/v2.2.0/diesel_cli-installer.sh | sh && sh $HOME/.cargo/env
# set the startup command to run your binary
CMD ["diesel migration run --locked-schema;./metools_backend"]
