# Casbin介绍

 Casbin 是一个强大的、高效的开源访问控制库，用于在应用程序中实现访问控制机制，如角色访问控制（RBAC）、属性访问控制（Attribute-Based Access Control，ABAC）、基于资源的访问控制等。Casbin 支持多种编程语言，包括 Go、Java、Node.js、PHP、Python、.NET 等，因此可以在不同的系统和应用程序中广泛使用。

 Casbin 的核心是一个访问控制模型，它定义了访问控制策略的结构和语义。这个模型由两部分组成：模型（Model）和策略（Policy）。

官网: <https://casbin.org/docs/en/overview>

## 模型（Model）

模型定义了访问控制的基本结构，通常包括以下几个部分：

1. 请求定义（Request definition）: 描述了一个访问请求的基本元素，例如 sub（主体，如用户）、obj（对象，如资源）、act（动作，如读取或写入）。
2. 策略定义（Policy definition）: 定义了策略中的元素，通常与请求定义相对应。
3. 角色定义（Role definition）: 定义了角色之间的继承关系和权限共享。
4. 策略效果（Policy effect）: 定义了当多个策略规则适用时，如何决定最终的访问控制效果（允许或拒绝）。
5. 匹配器（Matcher）: 定义了如何将请求与策略规则进行匹配，并决定是否允许该请求。

Casbin 使用一种简单的配置语言来定义模型，通常保存在 .conf 文件中。

## 策略（Policy）

策略是具体的访问控制规则，它们根据模型中定义的结构来指定。策略通常存储在文件、数据库或其他持久化存储中。策略文件是一个简单的 CSV 文件，其中列出了符合模型定义的规则。

## 特点

Casbin 的几个关键特点包括：

- **支持多种访问控制模型**：Casbin 支持多种访问控制模型，如 RBAC、ABAC、基于资源的访问控制等。

- **灵活性和可扩展性**：用户可以自定义访问控制模型，以满足特定的业务需求。

- **性能**：Casbin 在设计时考虑了性能，可以高效地处理访问控制请求。

- **多语言支持**：Casbin 有多种编程语言的实现，可以轻松集成到不同的应用程序中。

## 使用场景

Casbin 可以用于多种场景，包括但不限于：

- Web 应用程序的权限管理

- 微服务架构中的服务间访问控制

- 云服务和多租户系统的访问控制

- 文件和数据的访问权限管理

## 示例

以下是一个简单的 Casbin 模型和策略的例子：

### 模型文件（model.conf）

```text
[request_definition]
r = sub, obj, act

[policy_definition]
p = sub, obj, act

[role_definition]
g = _, _

[policy_effect]
e = some(where (p.eft == allow))

[matchers]
m = g(r.sub, p.sub) && r.obj == p.obj && r.act == p.act
```

### 策略文件（policy.csv）

```text
p, alice, data1, read
p, bob, data2, write
g, alice, admin
```

在这个例子中，我们定义了一个简单的 RBAC 模型，其中 Alice 有权限读取 data1，Bob 有权限写入 data2，Alice 被赋予了 admin 角色。匹配器 m 确定了请求和策略之间的匹配规则。

Casbin 通过加载这些模型和策略文件，可以轻松地在应用程序中实施访问控制。开发者可以通过 Casbin 的 API 来检查特定的访问请求是否被允许。

## 结合接口权限示例

 接口权限可以通过角色与接口进行关联，并结合角色访问控制（RBAC）模型来实现。在RBAC模型中，权限不直接分配给用户，而是分配给角色，用户通过被分配到特定的角色来获得相应的权限。这种方法简化了权限管理，因为你只需要管理角色和它们的权限，而不是每个用户的权限。

以下是使用RBAC实现接口权限控制的一般步骤：

1. **定义角色**：在系统中创建不同的角色，如管理员、编辑、访客等。

2. **定义权限**：确定系统中的资源，例如不同的API接口，并定义对这些资源的操作权限，如读取、写入、删除等。

3. **角色与权限的关联**：将定义好的权限分配给相应的角色。例如，管理员角色可能有权限访问和修改所有接口，而编辑角色可能只有权限访问和修改内容相关的接口。

4. **用户与角色的关联**：将用户分配给一个或多个角色。用户通过其角色获得相应的权限。

5. **权限检查**：当用户尝试访问某个接口时，系统会检查用户所属角色是否具有相应的权限。如果有权限，则允许访问；如果没有权限，则拒绝访问。

在实际应用中，这个过程通常涉及到以下几个组件：

- **用户（User）**：系统的使用者，需要通过验证其身份来获得相应的权限。

- **角色（Role）**：一组权限的集合，代表了系统中的不同职责和权限级别。

- **权限（Permission）**：对系统中资源的访问控制，通常表示为对某个接口的访问能力。

- **资源（Resource）**：系统中受保护的部分，如API接口、文件、数据等。

例如，使用Casbin实现接口权限控制可能会有如下的配置：

### 模型文件（`model.conf`）

```text
[request_definition]
r = sub, obj, act

[policy_definition]
p = sub, obj, act

[role_definition]
g = _, _

[policy_effect]
e = some(where (p.eft == allow))

[matchers]
m = g(r.sub, p.sub) && r.obj == p.obj && r.act == p.act
```

#### [request_definition]

这部分定义了访问请求的结构。在这个例子中，一个请求由三个部分组成：

- sub：主体（Subject），通常指的是用户或者用户组。

- obj：对象（Object），通常指的是要访问的资源，如文件、数据或API端点。

- act：动作（Action），指的是对对象执行的操作，如读取（read）、写入（write）或删除（delete）。

#### [policy_definition]

这部分定义了策略规则的结构，它与请求定义相对应。在这个例子中，策略同样由三个部分组成：

- sub：在策略中，这表示允许执行某个动作的主体。

- obj：这表示策略适用的对象。

- act：这表示允许的动作。

#### [role_definition]

这部分定义了角色之间的继承关系。在这个例子中：

- g：通常用来表示角色（Group）的定义，g = _,_ 表示一个简单的角色继承关系，其中第一个_是子角色，第二个_是父角色。子角色继承父角色的权限。

#### [policy_effect]

这部分定义了当存在多个策略规则时，如何决定请求是否被允许。在这个例子中：

- e = some(where (p.eft == allow))：表示如果至少有一个策略规则允许请求，那么请求就被允许。这里p.eft是策略效果（Policy Effect），通常可以是allow或deny。如果没有明确指定eft，则默认为allow。

#### [matchers]

这部分定义了如何匹配请求和策略规则。在这个例子中：

> m = g(r.sub, p.sub) && r.obj == p.obj && r.act == p.act

这是一个表达式，用于确定请求是否应该被允许。它的含义是：

- g(r.sub, p.sub)：检查请求的主体r.sub是否拥有策略中定义的角色p.sub，或者是该角色的继承者。

- r.obj == p.obj：请求的对象必须与策略中定义的对象相匹配。

- r.act == p.act：请求的动作必须与策略中定义的动作相匹配。

- 只有当所有这些条件都满足时，请求才被允许。

### **策略文件（policy.csv）**

```text
p, admin, /api/user, GET
p, admin, /api/user, POST
p, editor, /api/content, GET
p, editor, /api/content, POST
g, alice, admin
g, bob, editor
```

在这个例子中，我们定义了两个角色：admin 和 editor。admin 角色可以对 /api/user 接口进行 GET 和 POST 操作，而 editor 角色可以对 /api/content 接口进行 GET 和 POST 操作。然后我们将用户 Alice 分配给 admin 角色，将用户 Bob 分配给 editor 角色。这样，Alice 可以访问用户相关的接口，Bob 可以访问内容相关的接口。

## **设计表结构**

将 policy.csv 文件设计成一个数据库表结构，你需要创建一个表来存储策略信息。这个表通常包含策略文件中的每一列作为字段。以下是一个简单的表结构设计，它反映了 policy.csv 文件中的数据：

```text
CREATE TABLE policies (
    id INT PRIMARY KEY AUTO_INCREMENT,
    ptype VARCHAR(10), -- 表示策略类型，例如 'p' 代表权限策略，'g' 代表角色继承关系
    subject VARCHAR(255), -- 表示主体，例如用户或角色名
    object VARCHAR(255), -- 表示对象，例如资源或接口路径
    action VARCHAR(255) -- 表示动作，例如 'read', 'write'
);
```

在这个表结构中，id 是主键，用于唯一标识每条策略记录。ptype 字段用于区分策略类型，subject、object 和 action 字段分别对应策略文件中的主体、对象和动作。

如果你的策略还包含角色继承关系，你可能还需要一个额外的表来存储这些关系：

```text
CREATE TABLE role_links (
    id INT PRIMARY KEY AUTO_INCREMENT,
    ptype VARCHAR(10), -- 表示策略类型，通常 'g' 代表角色继承关系
    role1 VARCHAR(255), -- 表示角色名
    role2 VARCHAR(255) -- 表示继承自的角色名
);
```

在这个 role_links 表中，role1 是子角色，而 role2 是父角色，表示 role1 继承自 role2 的权限。

请注意，这些表结构是基于一个简单的策略文件设计的。根据你的具体需求，你可能需要添加更多的字段或表来存储额外的信息，例如策略的生效时间、创建者等。此外，字段类型和长度应根据实际情况进行调整以适应数据的大小和类型。

role_links 大概是可以使用用户与角色的关系表进行替代：

是的，role_links 表可以被视为用户与角色关系表，用于表示用户分配到的角色，或者角色与角色之间的继承关系。在实际应用中，这个表通常被称为“用户角色关系表”或“角色继承表”，具体取决于它所表示的关系类型。

如果表用于存储用户和角色之间的关系，它可能看起来像这样：

```text
CREATE TABLE user_roles (
    id INT PRIMARY KEY AUTO_INCREMENT,
    user_id INT, -- 表示用户的唯一标识
    role_id INT -- 表示角色的唯一标识
);
```

在这个 user_roles 表中，user_id 字段关联到用户表的主键，而 role_id 字段关联到角色表的主键。这样可以很容易地查询哪些用户被分配了哪些角色，或者一个特定的用户有哪些角色。

如果表用于存储角色之间的继承关系，它可能看起来像这样：

```text
CREATE TABLE role_inheritance (
    id INT PRIMARY KEY AUTO_INCREMENT,
    child_role_id INT, -- 表示子角色的唯一标识
    parent_role_id INT -- 表示父角色的唯一标识
);
```

在这个 role_inheritance 表中，child_role_id 字段关联到角色表中子角色的主键，而 parent_role_id 字段关联到角色表中父角色的主键。这样可以定义一个角色可以继承另一个角色的权限。

在设计数据库模式时，确保正确地设置外键约束以维护数据的完整性，并为常用的查询创建索引以提高查询性能。此外，根据实际需求，你可能还需要考虑其他因素，如多对多关系、角色和权限的多级继承等。
