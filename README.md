# Acrix-Admin-Web

这个是一个后端接口服务，同时内嵌静态文件服务。

## 框架技术栈

### 后端

- 语言: Rust
- 后端框架：Actix-web
- 数据库框架: Sea-Orm
- 日志框架: Tracing

### 前端

- 语言：
- 构建工具: Vite
- UI 框架：

## 框架功能列表

### 后端

- [x] 热重启
- [x] 内嵌 web 服务
- [ ] 日志
  - [x] 终端日志
  - [x] 文件日志
  - [x] 数据库日志
  - [ ] OpenTelemetry 日志
- [x] 自定义业务状态码
- [x] 数据库
  - [x] 自动迁移库表
  - [x] 读写数据库
- [x] 依赖注入
- [ ] 中间件
  - [x] 跨域
  - [x] Request ID
  - [x] Actix Request Identifier Middleware
  - [x] Tracing Logger
  - [x] JWT 令牌
  - [ ] 访问频率限制
  - [ ] 访问 IP 限制
  - [ ] OpenApi 鉴权
- [ ] API 文档
  - [ ] ApiPost 接口工具
  - [ ] 内置接口文档
- [ ] 定时任务调度
  - [ ] 即时任务
  - [ ] 定时任务
- [ ] 插件
  - [ ] 服务启动 logo
  - [x] 请求参数校验插件
  - [ ] [pprof]性能剖析工具
  - [ ] [Prometheus] 指标记录
  - [ ] [Swagger]接口文档, apipost 工具代替
  - [ ] 服务启动后打开浏览器
- [ ] 动态 SEO 优化
- [ ] 内存缓存
- [ ] 订阅
- [ ] [cron] 定时任务，在后台可界面配置
- [ ] [websocket]实时通讯

## 业务列表

- [x] 获取验证码
- [x] 用户注册
- [x] 登陆
  - [ ] 单点登录
- [x] 用户中心
- [ ] 权限管理
  - [x] 角色管理
  - [x] 用户管理
  - [x] 用户角色关系管理
  - [x] 部门管理
  - [x] 部门角色关系管理
  - [ ] 菜单管理
  - [ ] 菜单角色关系管理
- [ ] OpenApi 管理
  - [ ] OpenApi 接口管理
  - [ ] OpenApi 权限管理
- [ ] 系统管理
  - [ ] 全局配置管理
  - [x] 验证码管理
    - [ ] 已经使用过的验证码应该设置为过期
- [ ] 日志管理
  - [x] 系统日志
  - [ ] API 操作日志表
  - [ ] 前端日志
  - [x] 登陆日志管理
    - [ ] 操作日志类型
    - [ ] 状态: 成功/失败
      - [ ] 新增一条禁用的登陆日志？
- [ ] 数据中心
  - [ ] 数据字典维度管理
  - [ ] 数据字典管理
- [ ] 前端权限
  - [ ] 动态路由
  - [ ] 按钮权限
- [ ] 系统监控

## 待办

- 调研 log span 全局使用事件，放置在中间件中，这样日志中可以获取统一的 span 数据；
- web 日志，接入框架；
- 定时任务集成；
- doc 文档，细化为接口文档；
- 用户权限封装；
- 配置表/API 操作日志表/字典管理
  log span 的使用看能不能使用事件的调研，放置在中间件，监听整个流程；
  api 接口操作日志
  web 日志，看是否可以接入框架；
  定时任务调研与集成，有参考项目；
  权限封装；
  路由匹配
  状态还是需要使用 ID 进行透传，同时 body 取消 id 传入；
  数据状态调整为i8

https://github.com/actix/actix-extras/tree/master/actix-identity
https://crates.io/crates/actix-ws
https://crates.io/crates/actix-session
https://github.com/actix/actix-extras/tree/master/actix-limitation
https://github.com/ddimaria/rust-actix-example

## 开发文档

- [开发环境搭建](./docs/开发环境搭建.md)
- [编译与部署](./docs/编译与部署.md)
- [Sea-Orm 使用指南](./docs/Sea-Orm使用指南.md)
- [问题答疑](./docs/Q&A.md)
