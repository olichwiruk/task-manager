FROM rust:1 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
COPY . .
ENV SQLX_OFFLINE=1
RUN cargo build --release --bin task-manager

FROM ubuntu:22.04 AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/task-manager /usr/local/bin
ENTRYPOINT ["/usr/local/bin/task-manager"]
