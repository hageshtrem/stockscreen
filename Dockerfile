FROM rust:latest as builder
WORKDIR /usr/src/stockskreen
COPY . .
RUN rustup component add rustfmt 
#RUN rustup target add x86_64-unknown-linux-musl
#RUN RUSTFLAGS='-C target-feature=+crt-static' cargo install --path . --target=x86_64-unknown-linux-musl
RUN cargo install --path .

FROM debian:stretch
COPY --from=builder /usr/local/cargo/bin/stockskreen /usr/local/bin/stockskreen
CMD ["stockskreen"]