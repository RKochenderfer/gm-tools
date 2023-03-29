# Using the official Rust image as a builder
FROM rust:1.68 as builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

# Using a minimal image for the runtime
FROM debian:buster-slim

COPY --from=builder /usr/src/app/target/release/gm-tools-api /usr/local/bin/gm-tools-api

CMD ["gm-tools-api"]