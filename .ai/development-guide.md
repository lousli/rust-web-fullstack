# AI 开发助手配置

## 项目上下文
- **项目类型**: 全栈 Web 应用
- **后端**: Rust + Actix Web
- **前端**: HTML + CSS + JavaScript
- **数据库**: 未来可能添加 PostgreSQL 或 SQLite
- **部署**: 计划使用 Docker

## 代码生成偏好
1. **优先简洁性**: 生成简洁、可读的代码
2. **错误处理**: 总是包含适当的错误处理
3. **文档**: 为公共函数和复杂逻辑提供文档
4. **测试**: 建议包含基本的测试用例

## 架构决策
- 使用模块化设计
- 分离关注点（handlers, models, routes）
- RESTful API 设计
- 状态管理使用 Actix Web 的应用状态

## 常用模式
```rust
// Rust 错误处理模式
type Result<T> = std::result::Result<T, actix_web::Error>;

// 异步处理器函数
async fn handler(req: HttpRequest) -> Result<HttpResponse> {
    // 处理逻辑
    Ok(HttpResponse::Ok().json(response))
}
```

```javascript
// 前端 API 调用模式
async function apiCall(endpoint, options = {}) {
    try {
        const response = await fetch(`/api/${endpoint}`, {
            headers: { 'Content-Type': 'application/json' },
            ...options
        });
        return await response.json();
    } catch (error) {
        console.error('API call failed:', error);
        throw error;
    }
}
```

## 避免的模式
- 避免深度嵌套的回调
- 避免全局变量
- 避免不必要的复杂性
- 避免硬编码配置值
