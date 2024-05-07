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

### 后端框架

后端框架功能列表。

- [x] 热重启
- [x] 内嵌 Web 服务
- [ ] 日志
  - [x] 终端日志
  - [x] 文件日志
  - [x] 数据库日志
  - [ ] OpenTelemetry 日志
- [x] 自定义业务状态码
- [x] 数据库
  - [x] 迁移库表
  - [x] 读写数据库
  - [x] mock 单元测试
- [x] 依赖注入
- [ ] 中间件
  - [x] 跨域
  - [x] Actix Request Identifier
  - [x] Tracing Logger
  - [x] JWT 令牌
  - [x] OpenApi 鉴权
  - [ ] 访问频率限制
  - [ ] 访问 IP 限制
  - [x] Api 操作日志
- [ ] API 文档
  - [ ] ApiPost 接口工具
  - [ ] 内置接口文档
- [ ] 插件
  - [ ] 服务启动 Logo
  - [x] 请求参数校验插件
  - [ ] [pprof]性能剖析工具
  - [ ] [Prometheus] 指标记录
  - [ ] [Swagger]接口文档, apipost 工具代替
  - [ ] 服务启动后打开浏览器
- [ ] 动态 SEO 优化
- [x] 内存缓存
- [ ] 订阅
- [ ] cron定时任务
  - [x] 定时任务调度
    - [x] 即时任务
    - [x] 定时任务
  - [x] 系统任务
  - [ ] 用户任务
- [ ] [websocket]实时通讯

## 业务列表

- [x] 认证管理
  - [x] 获取验证码
  - [x] 用户注册
  - [x] 登陆
    - [ ] 单点登录
- [x] 用户中心
- [x] 权限管理
  - [x] 角色管理
    - [ ] 同步配置菜单权限
    - [ ] openapi 接口权限
    - [ ] 菜单权限
  - [x] 用户管理
  - [x] 部门管理
  - [x] 菜单管理
  - [x] 令牌管理
  - [x] OpenApi 接口管理
- [x] 系统管理
  - [x] 配置管理
  - [x] 验证码管理
  - [x] ICON 图标管理
  - [x] 数据字典管理
  - [x] 定时任务管理
- [ ] 日志管理
  - [x] 系统日志
  - [x] 操作日志
  - [ ] 前端日志
  - [x] 登陆日志
- [ ] 前端权限
  - [ ] 动态路由
  - [ ] 按钮权限
- [ ] 系统监控

## 待办

- 调研 log span 全局使用事件，放置在中间件中，这样日志中可以获取统一的 span 数据；
- 定时器-用户任务
- web 日志，接入框架；
- doc 文档，细化为接口文档；
- 用户权限封装；
- 接口鉴权
  - 路由匹配
  - open api 接口 rbac
  - 系统用户接口鉴权
- 用户分享码 share_code
- 用户会员等级
- 调度任务接口验证
  - 任务运行失败，日志回收
  - 耗时
  - 状态

## 表划分域

用户与身份管理：
●t_user_profile：用户资料信息
●t_user_email — 用户邮箱表 (UserEmail)
●t_user_phone — 用户手机号表 (UserPhoneNumber)
●t_user_login_log — 用户登录日志表 (UserLoginLog)
●t_user_blockchain_wallet — 用户区块链钱包表
●t_user_location — 用户地理位置表 (UserLocation)

权限与访问控制：
●t_auth_position — 岗位表 (Position)
●t_auth_rank — 职级表 (Rank)

会员系统：
●t_member_level — 会员等级表 (MemberLevel)

日志与审计：
●t_audit_api_operation_log — API操作日志表 (ApiOperationLog)
●t_audit_system_log — 系统日志表 (SystemLog)
●t_audit_web_log — WEB日志表 (WebLog)

任务调度与后台作业：
●t_task_scheduled — 调度任务表 (ScheduledTask)
●t_task_scheduled_status_log — 调度任务状态日志表 (ScheduledTaskStatusLog)
●t_task_scheduled_event_log — 调度任务事件日志表 (ScheduledTaskEventLog)

系统管理与基础设施：
●t_sys_verification_code — 验证码表 (VerificationCode)
●t_sys_config — 配置表 (Configuration)
●t_sys_image_resource — 图片资源表 (ImageResource)
●t_sys_dict_dimension — 字典维度表 (DictionaryDimension)
●t_sys_dict_data — 字典数据表 (DictionaryData)

个人资料信息 (User Profile Information)
个人资料信息则更侧重于描述用户的个人特征和偏好。这些信息用于个性化服务、用户画像分析和增强用户体验。个人资料信息通常包括：
●出生日期（Date of Birth）：用户的出生年月日。
●地址（Address）：用户的居住或邮寄地址。
●偏好设置（Preferences）：用户的偏好设置，如语言、主题等。

路由整理
部门管理
部门角色关系管理   部门ID   department_id -》department_id
令牌管理
令牌角色关系管理

OpenApi接口角色关系 napi_id  ->  openapi_id

用户权限前最

<https://github.com/actix/actix-extras/tree/master/actix-identity>
<https://crates.io/crates/actix-ws>
<https://crates.io/crates/actix-session>
<https://github.com/actix/actix-extras/tree/master/actix-limitation>
<https://github.com/ddimaria/rust-actix-example>

## 开发文档

- [开发环境搭建](./docs/开发环境搭建.md)
- [编译与部署](./docs/编译与部署.md)
- [Sea-Orm 使用指南](./docs/Sea-Orm使用指南.md)
- [问题答疑](./docs/Q&A.md)
