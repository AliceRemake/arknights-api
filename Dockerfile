FROM rust:1.80.1 AS chef

RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin arknights-api

FROM debian:bookworm-slim AS runtime
RUN apt update -o Acquire::http::No-Cache=True \
    && apt install -y libssl3 -o Acquire::http::No-Cache=True \
    && apt install -y git -o Acquire::http::No-Cache=True \
    && git config --global http.sslverify "false" \
    && git config --global https.sslverify "false"

WORKDIR /app
COPY --from=builder /app/target/release/arknights-api /root

ENTRYPOINT [ "/root/arknights-api" ]