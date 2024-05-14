# Casbin介绍

Casbin是一个开源的访问控制库，用于实现基于属性的访问控制（Attribute-Based Access Control，ABAC）

Casbin的核心概念包括：

1. 策略模型（Policy Model）：策略模型定义了访问控制的结构和规则。Casbin支持多种策略模型，如基于角色的访问控制（Role-Based Access Control，RBAC）、基于属性的访问控制（Attribute-Based Access Control，ABAC）等。

2. 策略规则（Policy Rules）：策略规则是策略模型的具体实现。策略规则定义了允许或拒绝访问的条件。例如，在RBAC模型中，策略规则可以定义用户和角色之间的关系。

3. 请求上下文（Request Context）：请求上下文包含了访问控制的输入参数，如用户、资源和操作。Casbin使用请求上下文来检查策略规则是否允许访问。

4. 策略存储（Policy Storage）：策略存储用于存储策略规则。Casbin支持多种策略存储，如文件、数据库等。

Casbin的工作流程如下：

1. 定义策略模型和策略规则。

2. 将策略规则存储在策略存储中。

3. 在应用程序中使用Casbin库进行访问控制检查。

4. Casbin库根据请求上下文和策略规则进行访问控制判断。

Casbin支持多种编程语言，如Go、Java、Python、Node.js等。您可以在Casbin官方文档中找到更多关于Casbin的信息和示例：<https://casbin.org/docs/en/overview>

## RBAC示例

```text
package main

import (
 "fmt"
 "github.com/casbin/casbin/v2"
 "github.com/casbin/casbin/v2/model"
 "github.com/casbin/casbin/v2/persist/file-adapter"
)

func main() {
 // 定义策略模型
 modelText := `
[request_definition]
r = sub, obj, act

[policy_definition]
p = sub, obj, act

[policy_effect]
e = some(where (p.eft == allow))

[matchers]
m = r.sub == p.sub && r.obj == p.obj && r.act == p.act
`

 // 加载策略模型
 model, err := model.NewModelFromString(modelText)
 if err != nil {
  panic(err)
 }

 // 加载策略规则
 adapter, err := fileadapter.NewAdapter("policy.csv")
 if err != nil {
  panic(err)
 }

 // 创建Casbin实例
 enforcer, err := casbin.NewEnforcer(model, adapter)
 if err != nil {
  panic(err)
 }

 // 添加策略规则
 enforcer.AddPolicy("alice", "data1", "read")
 enforcer.AddPolicy("bob", "data2", "write")
 enforcer.AddPolicy("data_group", "data1", "read")
 enforcer.AddPolicy("data_group", "data2", "write")
 enforcer.AddGroupingPolicy("alice", "data_group")

 // 检查访问权限
 result, err := enforcer.Enforce("alice", "data1", "read")
 if err != nil {
  panic(err)
 }
 fmt.Printf("Alice has permission to read data1: %v\n", result)

 result, err = enforcer.Enforce("bob", "data2", "write")
 if err != nil {
  panic(err)
 }
 fmt.Printf("Bob has permission to write data2: %v\n", result)

 result, err = enforcer.Enforce("alice", "data2", "write")
 if err != nil {
  panic(err)
 }
 fmt.Printf("Alice has permission to write data2: %v\n", result)
}
```

1. 定义策略模型：我们定义了一个简单的策略模型，其中包含请求定义（request_definition）、策略定义（policy_definition）、策略效果（policy_effect）和匹配器（matchers）。策略模型定义了访问控制的结构和规则。

2. 加载策略模型：我们使用model.NewModelFromString函数从字符串中加载策略模型。

3. 加载策略规则：我们使用fileadapter.NewAdapter函数创建一个文件适配器，用于从policy.csv文件中加载策略规则。

4. 创建Casbin实例：我们使用casbin.NewEnforcer函数创建一个Casbin实例，传入策略模型和策略规则适配器。

5. 添加策略规则：我们使用enforcer.AddPolicy方法添加了一些策略规则，如Alice可以读取data1，Bob可以写入data2，以及一个名为data_group的角色可以读取data1和写入data2。我们还使用enforcer.AddGroupingPolicy方法将Alice添加到data_group角色中。

6. 检查访问权限：我们使用enforcer.Enforce方法检查Alice和Bob是否具有访问特定资源的权限。例如，我们检查Alice是否可以读取data1，Bob是否可以写入data2，以及Alice是否可以写入data2。

这个示例展示了如何使用Go语言和Casbin库实现基于角色的访问控制（RBAC）。您可以根据自己的需求修改策略模型、策略规则和访问控制检查。

## 结合接口权限RBAC示例

要实现基于角色的接口权限管理，您可以在Casbin中使用RBAC模型

1. 定义策略模型：在策略模型中，我们需要定义请求（request）和策略（policy）的结构。请求包含用户、角色、接口和操作，策略包含角色、接口和操作。

 ```text
 [request_definition]

 r = sub, role, obj, act

 [policy_definition]

 p = role, obj, act
 ```

2. 添加策略规则：为每个角色定义可以访问的接口和操作。例如，您可以为管理员角色定义以下策略规则：

```text
p, admin, /api/users, GET

p, admin, /api/users, POST

p, admin, /api/users, PUT

p, admin, /api/users, DELETE
```

这些规则表示管理员角色可以对/api/users接口执行GET、POST、PUT和DELETE操作。

1. 添加角色分配：将用户分配给相应的角色。例如，您可以将Alice分配给管理员角色：

```text
g, alice, admin
```

4. 检查访问权限：使用enforcer.Enforce方法检查用户是否具有访问特定接口的权限。例如，您可以检查Alice是否可以对/api/users接口执行GET操作：

```go
result, err := enforcer.Enforce("alice", "admin", "/api/users", "GET")
if err != nil {
 panic(err)
}
fmt.Printf("Alice has permission to access /api/users with GET method: %v\n", result)
```

这个示例展示了如何使用Casbin实现基于角色的接口权限管理。您可以根据自己的需求修改策略模型、策略规则和访问控制检查。

## **ABAC 示例**

ABAC（Attribute-Based Access Control，基于属性的访问控制）是一种基于用户、资源和环境属性的访问控制方法

1. 定义策略模型：在策略模型中，我们需要定义请求（request）和策略（policy）的结构。请求包含用户、资源和操作，策略包含属性和条件。

```text
[request_definition]

r = sub, obj, act

[policy_definition]

p = attr, cond
```

2. 添加策略规则：为每个属性定义访问条件。例如，您可以为管理员角色定义以下策略规则：

```text
p, role==admin, obj=="/api/users" && act=="GET"

p, role==admin, obj=="/api/users" && act=="POST"

p, role==admin, obj=="/api/users" && act=="PUT"

p, role==admin, obj=="/api/users" && act=="DELETE"
```

这些规则表示管理员角色可以对/api/users接口执行GET、POST、PUT和DELETE操作。

3. 添加属性：为用户和资源定义属性。例如，您可以为Alice定义以下属性：

```text
[user_alice]

role = admin
```

这个属性表示Alice的角色是管理员。

4. 检查访问权限：使用enforcer.Enforce方法检查用户是否具有访问特定接口的权限。例如，您可以检查Alice是否可以对/api/users接口执行GET操作：

```text
result, err := enforcer.Enforce("user_alice", "/api/users", "GET")
if err != nil {
 panic(err)
}
fmt.Printf("Alice has permission to access /api/users with GET method: %v\n", result)
```

这个示例展示了如何使用Casbin实现基于属性的接口权限管理。您可以根据自己的需求修改策略模型、策略规则和访问控制检查。

需要注意的是，ABAC的策略规则和属性可以更加灵活和复杂，以满足不同的访问控制需求。您可以在Casbin文档中找到更多关于ABAC的示例和解释：<https://casbin.org/docs/en/abac>
