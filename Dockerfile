FROM rust:slim AS build

# create a new empty shell project
RUN USER=root cargo new --bin backend
WORKDIR /backend

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./.sqlx ./.sqlx
COPY ./migrations ./migrations

# build for release
RUN rm ./target/release/deps/backend*
RUN cargo build --release

COPY ./.env ./.env

# our final base
FROM debian:bookworm-slim

# copy the build artifact from the build stage
COPY --from=build /backend/target/release/backend .
COPY --from=build /backend/.env ./.env

# set the startup command to run your binary
ENTRYPOINT ["./backend"]
