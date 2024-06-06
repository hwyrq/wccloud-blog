FROM rust
EXPOSE 8085

WORKDIR wccloud-web-rust
COPY Cargo.toml .
COPY src .
RUN pwd
RUN ls

RUN cargo build --release
CMD ./target/release/wccloud-web-rust
