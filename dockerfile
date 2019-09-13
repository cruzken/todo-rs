FROM rust:1.37 as build

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update
RUN apt-get install -y libsqlite3-dev

# Create new project to cache dependencies
WORKDIR /usr/src
RUN USER=root cargo new --bin todo
WORKDIR /usr/src/todo
COPY app/Cargo.* ./

# Build dependencies
RUN cargo build --release

# Copy sources and build release
RUN rm -r src
RUN rm -r target/release/deps/todo*
COPY app/src ./src
RUN cargo build --release
