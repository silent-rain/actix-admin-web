# 后端服务

这个是一个后端接口服务，同时内嵌静态文件服务。

## 编译与调试

### 代码检查

```shell
cargo clippy
# or
cargo clippy -p admin
```

### 调试模式

```shell
cd server/admin

cargo run -p admin
```

### 生产模式

```shell
cd server/admin

cargo build -p admin
```

### 查询依赖

```shell
cd server/admin

cargo +nightly udeps -p admin

cargo +nightly udeps --workspace
```

## 开发文档

```shell
cd server/admin

cargo doc -p admin
```

## 服务

- [前端服务](http://127.0.0.1:8000/)
- [后端服务](http://127.0.0.1:8000/api/v1/)
- [swagger-ui](http://127.0.0.1:8000/swagger-ui/)

## 参考文档

- [sea-orm](https://www.sea-ql.org/SeaORM/docs/index/)
- [actix-web](https://actix.rs/docs/handlers)
- [validation](https://dev.to/chaudharypraveen98/form-validation-in-rust-404l)
- [validator](https://lib.rs/crates/validator)
