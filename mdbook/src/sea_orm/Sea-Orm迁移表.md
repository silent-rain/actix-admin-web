# Sea-Orm迁移表

## 安装依赖

```shell
cargo install sea-orm-cli
```

## 配置环境变量文件

编辑环境变量文件，修改为目标数据库地址：
> vim service/.env

```text
DATABASE_URL=sqlite://data.dat?mode=rwc

or

DATABASE_URL=mysql://user:pass@127.0.0.1:3306/actix_admin_web
```

## 库表创建迁移实体

### 初始化迁移目录

```shell
# 进入core 目录
cd server/crates/core

# 初始化迁移目录
sea-orm-cli migrate init
# or 指定移目录
sea-orm-cli migrate init -d ./migration
```

### 创建迁移实体文件

- 如果您已经有一个包含表和数据的数据库，则可以跳过

```shell
# 进入core 目录
cd server/crates/core

# 指定sqlite数据库创建实体文件
sea-orm-cli migrate generate -u sqlite://../data.db create_table

# .env 的环境变量文件
sea-orm-cli migrate generate create_table
```

### 根据数据库生成实体文件

- 注意会覆盖原始文件

```shell
# 进入core 目录
cd service/core

# 指定数据库生成实体
sea-orm-cli generate entity -u sqlite://../data.db -o entity/src
sea-orm-cli generate entity --database-url=mysql://one:pass@localhost/actix_admin_web -o entity/src
```

## CLI 迁移的表文件

```shell
# 进入迁移目录
cd server/crates/core/migration

# 指定数据库进行迁移
# cargo run -- COMMAND
cargo run -- -u sqlite://../../web/data.dat up

# or 根据环境变量配置进行迁移
cargo run -- up

# or 项目根目录进行迁移
cd service
cargo run --package migration -- up
```

## 脚本迁移表

脚本迁移表文件且可以自行创建数据库。注意需要先配置 `.env` 文件。

```sh
# 进入迁移目录
cd server/crates/core/migration

# 开始迁移
cargo run --package migration --example migration
```

## 参考文档

- [sea-orm](https://www.sea-ql.org/SeaORM/docs/index/)
