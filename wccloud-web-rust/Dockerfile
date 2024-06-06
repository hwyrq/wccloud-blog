FROM rust
EXPOSE 8085

WORKDIR wccloud-web-rust
RUN pwd

RUN cargo build --release
CMD ./target/release/wccloud-web-rust
