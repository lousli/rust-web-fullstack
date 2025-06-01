# GitHub Copilot 自定义指令

## 项目概览
这是一个 Rust + Frontend 全栈项目，使用 Actix Web 作为后端框架，HTML/CSS/JavaScript 作为前端技术栈。

## 编码风格和最佳实践

### Rust 后端
- 使用 `actix-web` 框架构建 Web 服务
- 遵循 Rust 官方代码风格指南
- 使用模块化架构：handlers、routes、models 分离
- 错误处理使用 `Result<T, E>` 类型
- 异步函数使用 `async/await` 语法
- 依赖注入使用 Actix Web 的 Data 提取器

### 前端开发
- 使用原生 JavaScript，避免复杂框架
- CSS 使用现代特性（Grid、Flexbox、CSS 变量）
- 保持简洁的 HTML 结构
- 使用语义化标签
- 响应式设计优先

### 代码注释
- Rust 函数使用文档注释 `///`
- JavaScript 使用 JSDoc 格式
- 复杂逻辑添加行内注释说明

### 文件命名规范
- Rust 文件使用 snake_case
- 前端文件使用 kebab-case
- 常量使用 SCREAMING_SNAKE_CASE

## 项目特定要求
- 后端提供静态文件服务
- API 路由以 `/api/` 开头
- 使用 JSON 格式进行前后端通信
- 错误响应包含标准化的错误码和消息

## 安全考虑
- 输入验证和清理
- CORS 配置
- 避免硬编码敏感信息
- 使用环境变量配置

## 测试要求
- Rust 单元测试使用 `#[cfg(test)]` 模块
- 集成测试放在 `tests/` 目录
- 前端测试使用简单的断言

## 性能优化
- 静态资源压缩和缓存
- 数据库查询优化
- 异步处理长时间运行的任务
