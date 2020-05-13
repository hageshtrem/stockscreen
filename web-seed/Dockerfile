FROM rust:latest as builder
RUN cargo install cargo-make
RUN cargo install microserver
WORKDIR /usr/src/web-seed
COPY . .
# Need to set env API_URL="http://localhost/v1"
ARG API_URL
ENV API_URL $API_URL
RUN cargo make build
# Need to set env PORT="80"
CMD ["cargo", "make", "serve"]