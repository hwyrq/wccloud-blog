FROM rust
EXPOSE 8085

WORKDIR wccloud-web-rust
COPY Cargo.toml /wccloud-web-rust
COPY src /wccloud-web-rust
RUN pwd
RUN ls

RUN cargo build --release
CMD ./target/release/wccloud-web-rust
