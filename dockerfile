FROM rust:latest as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/CompareItBackend .

CMD ["./CompareItBackend"]