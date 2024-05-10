# RESTful API 设计(GPT)

## 接口命名

接口命名应该简洁、直观且遵循一定的标准。根据 RESTful 设计原则，这里有一些建议来优化您的接口命名：

- 使用小写字母和中划线来分隔单词，而不是驼峰式或下划线。
- 资源名称应该使用复数形式，以表示资源集合。这是因为每个 URL 代表了一种资源类型，而资源通常是以集合的形式存在的。
- HTTP 方法应该准确反映操作的意图，例如 GET 用于获取数据，POST 用于创建数据，PUT 用于更新数据，DELETE 用于删除数据。
- 使用资源名称的复数形式（/roles 而不是 /role），以表示操作的是资源集合。
- 避免使用动词（如 /all 或 /list），因为 HTTP 方法（GET、POST、DELETE 等）已经表达了操作的意图。
- 对于创建和删除操作，应明确指定要操作的资源。

### 示例

- 例如，/users 表示用户的集合，而不仅仅是单个用户。这种命名方式有助于 API 的使用者理解他们正在与资源集合进行交互，即使是在请求单个资源的情况下。
- 此外，使用复数形式可以在语义上区分单个资源和资源集合。例如，/users/{id} 表示集合中的一个特定用户，而/user 可能会被误解为 API 的某种特殊功能或服务。
- 例如，如果用户已经通过身份验证并且持有一个 token，那么他们可以发送一个 GET 请求到 /users/me 或 /users/profile，服务器将返回与该 token 关联的用户的详细信息。这种方法避免了在 URL 中直接使用用户 ID，同时提供了一种简洁的方式来访问个人信息。

## 接口请求参数解析结构体命名

- 明确性：确保名称能够清楚地表达出其用途。例如，GetUserListReq 可以表示这是一个获取用户列表的请求。
- 简洁性：尽量避免冗长的名称，但同时要保持描述性。如果 Req 是您用来表示请求的常用缩写，那么可以保留。否则，可以考虑使用完整的单词 Request。
- 一致性：在整个项目中保持命名的一致性。如果您使用 Req 来表示请求，那么所有请求相关的结构体都应该使用这个后缀。
- 避免冗余：如果结构体已经在 request 模块中，那么可能不需要在名称中再次提到 Req。

对于增删改查的接口命名，您可以遵循以下约定：

- 增（Create）：通常使用 create 或 add 作为前缀，如 CreateUser 或 AddUser。
- 删（Delete）：通常使用 delete 或 remove 作为前缀，如 DeleteUser 或 RemoveUser。
- 改（Update）：通常使用 update 作为前缀，如 UpdateUser。
- 查（Read）：通常使用 get、fetch、find 或 query 作为前缀，如 GetUser、FetchUserDetails、FindUsers 或 QueryUserPermissions。
  - 结合 RESTful API 设计和 Rust 的命名规范，对于表示获取资源列表的请求，您可以使用复数形式，如 GetUsersReq 或 UsersReq，这样既符合 RESTful 的习惯，也清晰地传达了这是一个获取多个用户的请求。
  - 如果您的项目中通常使用 List 来表示集合，那么 GetUserListReq 可能更合适；如果您倾向于使用复数形式来表示集合，那么 GetUsersReq 可能是更好的选择。

## 常用命名参考

### 获取验证码

- 路由命名
  - 通过 ID 获取验证码： GET /captchas/{id}
  - 通过 Captcha ID 获取验证码： GET /captchas/by-captcha-id/{captcha_id}
- 接口命名
  - 通过 ID 获取验证码： info
  - 通过 Captcha ID 获取验证码：get_captcha_by_captcha_id
- 请求参数结构体定义
  - 通过 ID 获取验证码： 路径匹配
  - 通过 Captcha ID 获取验证码：路径匹配

### 获取用户详情

- 路由命名
  - 通过 ID 获取用户详情： GET /users/{id}
  - 通过 ID 获取用户手机号码： GET /users/{id}/phone
  - 获取用户列表： GET /users?all=true
  - 添加用户： POST /users
  - 删除用户： DELETE /users/{id}
  - 更新用户： PUT /users
- 接口命名
  - 通过 ID 获取用户详情： info
  - 通过 ID 获取用户手机号码： get_phone/get_user_phone
  - 获取用户列表：list
  - 添加用户： add_user
  - 删除用户： delete_user/del_user
  - 更新用户： update_user
- 请求参数结构体定义
  - 通过 ID 获取用户详情： 路径匹配请求参数
  - 通过 ID 获取用户手机号码：路径匹配请求参数 -> GetUserPhoneRsp
  - 获取用户列表：GetUsersReq/GetUserListReq -> GetUsersRsp/GetUserListRsp
  - 添加用户： AddUserReq
  - 删除用户：路径匹配请求参数
  - 更新用户：UpdateUserReq
