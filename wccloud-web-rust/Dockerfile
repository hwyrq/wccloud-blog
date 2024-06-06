FROM rust
EXPOSE 8085
#工作目录
WORKDIR wccloud-web-rust
#资源文件
COPY Cargo.toml /wccloud-web-rust/
#资源文件
COPY src /wccloud-web-rust/src
#cargo依赖缓存，防止每次都下载，会比较慢
COPY cargo /usr/local/cargo
COPY target /wccloud-web-rust/target

RUN cargo build --release
CMD ./target/release/wccloud-web-rust
