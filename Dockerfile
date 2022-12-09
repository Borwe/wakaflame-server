FROM rust:1-buster as builder

WORKDIR /usr/src/wakaflame-rs
COPY . .
RUN cargo build --release

FROM fedora:37

WORKDIR /usr/bin/
ENV PORT 8080

RUN dnf install -y openssl-devel
COPY --from=builder /usr/src/wakaflame-rs/target/release/wakaflame-rs /usr/bin/wakaflame-rs

CMD ["/usr/bin/wakaflame-rs"]
