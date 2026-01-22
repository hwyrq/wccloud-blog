# wccloud-gateway-rust

Rust实现的微服务网关，基于Actix-web框架开发，用于wccloud-blog项目。

## 项目特性

- 路由转发：支持将请求转发到不同的微服务
- 负载均衡：支持服务发现和负载均衡
- 过滤器：支持StripPrefix等请求过滤功能
- 配置中心：集成Nacos，支持动态配置管理
- 高性能：基于Rust语言实现，具备优异的性能

## 依赖

- Rust 1.75+
- Nacos 服务器

## 架构

- `actix-web` - Web框架
- `nacos-sdk` - 服务发现和配置管理
- `reqwest` - HTTP客户端
- `serde` - 数据序列化与反序列化
- `dashmap` - 线程安全的哈希表，用于存储路由表

## 配置

应用支持从Nacos配置中心获取配置信息，主要包括:
- 服务器端口配置
- Nacos连接配置
- 路由规则配置

## 运行

```bash
cargo run
```

或构建后运行:

```bash
cargo build --release
./target/release/wccloud-gateway-rust
```

## Docker部署

```bash
docker build -t wccloud-gateway-rust .
docker run -p 8081:8081 wccloud-gateway-rust
```

## 路由配置说明

```yaml
routes:
  - id: auth # 路由ID
    uri: lb://wccloud-auth-server # 目标服务
    predicates: # 匹配条件
      - path: /wccloud-auth-server/**
    filters: # 过滤器
      - strip_prefix: 1 # 去除前缀
```

## 环境配置

- `server.port` - 网关监听端口
- `nacos` 配置项包含Nacos服务器地址、命名空间、用户名密码等信息
- `routes` - 定义路由规则列表