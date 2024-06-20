#因为是在jenkins内构建的，所以这里为了兼容也让rust在jenkins镜像运行
FROM registry.cn-shanghai.aliyuncs.com/hwyrq/jenkins:lts
EXPOSE 8085
COPY application-pro.yml ./application.yml
COPY ./target/release/wccloud-web-rust ./wccloud-web-rust
CMD ./wccloud-web-rust
