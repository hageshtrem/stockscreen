FROM rust-image as builder
WORKDIR /usr/src/rust-image
COPY . .
RUN cargo install --path .

FROM debian:stretch
RUN apt-get update && apt-get install -y openssl
COPY --from=builder /usr/local/cargo/bin/emitent-service /usr/local/bin/emitent-service
CMD ["emitent-service"]