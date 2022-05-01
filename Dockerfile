FROM rust:1-buster

WORKDIR /usr/src/wakaflame-rs
COPY . .
RUN cargo install --path .
CMD ["wakaflame-rs"]
