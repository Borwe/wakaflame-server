FROM rust:1-bullseye as builder

WORKDIR /usr/src/wakaflame-rs
COPY . .
RUN cargo build --release

FROM  debian:bullseye

RUN apt-get update
RUN apt-get install -y openssl ca-certificates
WORKDIR /usr/bin/
ENV PORT 8080
COPY --from=builder /usr/src/wakaflame-rs/target/release/ /usr/bin/wakaflame-rs

CMD ["/usr/bin/wakaflame-rs/wakaflame-rs"]
