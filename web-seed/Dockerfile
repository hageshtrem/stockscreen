FROM rust-image
WORKDIR /usr/src/rust-image
COPY . .
# Need to set env API_URL="http://localhost/v1"
ARG API_URL
ENV API_URL $API_URL
RUN cargo make build
# Need to set env PORT="80"
CMD ["cargo", "make", "serve"]