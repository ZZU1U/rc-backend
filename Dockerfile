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
COPY . .

# build for release
RUN rm ./target/release/deps/backend*
RUN cargo build --release

# our final base
FROM debian:12-slim

# copy the build artifact from the build stage
COPY --from=build /backend/target/release/backend .

# set the startup command to run your binary
ENTRYPOINT ["./backend"]
