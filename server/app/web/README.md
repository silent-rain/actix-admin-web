# WEB 端服务

这个是一个后端接口服务，同时内嵌静态文件服务。

## 运行服务

### 调试模式

```shell
cargo run --package web
```

### 生产模式

```shell
cargo build -r --package web
```

## 自动重新加载开发服务器

### 安装依赖

```shell
cargo install systemfd cargo-watch
```

### 热重启

```shell
cargo watch -d 2 -x clippy -x run --package web
```

### 以套接字的方式热重启

```shell
systemfd --no-pid -s http::3000 -- cargo watch -x run --package web
```

## 参考文档

- [sea-orm](https://www.sea-ql.org/SeaORM/docs/index/)
- [actix-web](https://actix.rs/docs/handlers)
- [validation](https://dev.to/chaudharypraveen98/form-validation-in-rust-404l)
