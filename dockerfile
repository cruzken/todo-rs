FROM rust:1.37 as build

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update
RUN apt-get install -y libsqlite3-dev sqlite3

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

# Preparing app directory
RUN mkdir /todo
RUN cp target/release/backend /todo/

# Create database
COPY app/migrations/task/up.sql .
RUN sqlite3 /todo/testdb.sqlite3 < up.sql

#Copy assets
COPY app/static /todo/static

# Expose required port
EXPOSE 8088

WORKDIR /todo
