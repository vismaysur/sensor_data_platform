# build stage
FROM rust:1.91 AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs && cargo build --release && rm -r src

COPY src ./src
RUN cargo build --release

# deployment stage
FROM debian:trixie-slim

RUN apt-get update && apt-get install -y \
    libsqlite3-0 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/sensor /usr/local/bin/sensor
RUN mkdir /data

CMD ["sensor", "--sensor-id", "0"]