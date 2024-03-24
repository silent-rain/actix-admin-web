# Sea-Orm 使用指南

## 安装依赖

```shell
cargo install sea-orm-cli
```

## 库表迁移

### 初始化迁移目录

```shell
# 进入core 目录
cd service/core

# 初始化迁移目录
sea-orm-cli migrate init
# or 指定移目录
sea-orm-cli migrate init -d ./migration
```

### 创建迁移

- 如果您已经有一个包含表和数据的数据库，则可以跳过

```shell
# 进入core 目录
cd service/core

# 指定数据库创建迁移文件
sea-orm-cli migrate generate -u sqlite://../data.db create_table
# or 存在 .env 的环境变量文件
sea-orm-cli migrate generate create_table
```

- 环境变量文件: .env
  - 编辑环境变量文件： vim service/.env

```
# DATABASE_URL=mysql://user:pass@localhost/db_name
DATABASE_URL=sqlite://data.dat
```

### CLI 迁移

```shell
# 进入迁移目录
cd server/core/migration

# 指定数据库进行迁移
# cargo run -- COMMAND
cargo run -- -u sqlite://../../web/data.dat up

# or 根据环境变量配置进行迁移
cargo run -- up

# or 项目根目录进行迁移
cd service
cargo run --package migration -- up
```

## 根据数据库生成实体

- 注意会覆盖原始文件

```shell
# 进入core 目录
cd service/core

# 指定数据库生成实体
sea-orm-cli generate entity --database-url=mysql://one:pass@localhost/actix_admin_web -o entity/src
sea-orm-cli generate entity -u sqlite://data.db -o entity/src
```

## 参考文档

- [sea-orm](https://www.sea-ql.org/SeaORM/docs/index/)
