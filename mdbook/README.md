# 开发文档

## 安装 mdbook

安装 mdbook CLI 工具后，可以使用它来创建和渲染书籍。

```shell
cargo install mdbook
```

## 构建与运行书籍

### 构建书籍

```shell
mdbook build
```

## 构建并打开书籍

使用命令serve，它将构建书籍并启动本地网络服务器。
该serve命令监视书籍src目录的更改，重建书籍并为每次更改刷新客户端。

```shell
mdbook serve --open
```

### 自动触发构建

使用 mdbook watch 将会监视文件，并在修改文件时自动触发构建。

```shell
mdbook watch --open
```
