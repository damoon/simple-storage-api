FROM rust:1.48.0-slim-buster AS builder
WORKDIR /usr/src/app
COPY src src
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo install --path .

FROM debian:buster-slim
ENV TIDE_ADDR 0.0.0.0:8080
#RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/simple-storage-api /usr/local/bin/simple-storage-api
ENTRYPOINT ["simple-storage-api"]
