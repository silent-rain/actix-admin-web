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
  - [x] Tracing Logger
  - [x] Actix Request Identifier Middleware
  - [x] JWT 令牌
  - [ ] 访问频率限制
  - [ ] 访问 IP 限制
  - [ ] 内部接口鉴权
  - [ ] Public API 鉴权
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
  - [x] 用户管理
  - [x] 角色管理
  - [x] 用户与角色关系管理
  - [ ] 菜单管理
  - [ ] 部门管理
- [ ] API 鉴权管理
  - [ ] HTTP 接口管理
- [ ] 系统管理
  - [ ] 网站配置管理
  - [x] 验证码管理
    - [ ] 已经使用过的验证码应该设置为过期
- [ ] 日志管理
  - [x] 系统日志
  - [ ] 请求日志
  - [ ] 前端日志
  - [x] 登陆日志管理
    - [ ] 操作日志类型
    - [ ] 状态: 成功/失败
      - [ ] 新增一条禁用的登陆日志？
- [ ] 前端权限
  - [ ] 动态路由
  - [ ] 按钮权限
- [ ] 数据中心
  - [ ] 全局配置管理
  - [ ] 字典管理
- [ ] 系统监控

## 待办

## 开发文档

- [开发环境搭建](./docs/开发环境搭建.md)
- [编译与部署](./docs/编译与部署.md)
- [Sea-Orm 使用指南](./docs/Sea-Orm使用指南.md)
- [问题答疑](./docs/Q&A.md)
