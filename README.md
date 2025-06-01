# 🚀 Rust + Frontend 全栈项目

这是一个现代化的全栈 Web 开发项目，集成了 Rust 后端和前端，配合 AI 工具链。

## 📁 项目结构

```
📦 项目根目录
├── 🦀 rust-web-backend/     # Rust Web 后端服务器
│   ├── src/
│   │   ├── main.rs          # 应用入口
│   │   ├── handlers/        # 请求处理器
│   │   ├── routes/          # 路由定义
│   │   └── models/          # 数据模型
│   └── Cargo.toml           # Rust 项目配置
├── 🌐 frontend/             # 前端项目
│   ├── src/
│   │   ├── index.html       # 主页面
│   │   └── assets/          # 静态资源
│   └── package.json         # Node.js 项目配置
└── 📖 README.md             # 项目说明（本文件）
```

## 🛠️ 技术栈

- **后端**: Rust + Actix Web + Actix Files
- **前端**: HTML + CSS + JavaScript
- **AI 工具**: Model Context Protocol (MCP) + Anthropic Claude
- **开发工具**: VS Code + GitHub Copilot

## 🚀 快速开始

### 📋 环境要求

- Rust 1.70+ （安装：https://rustup.rs/）
- Node.js 18+ （安装：https://nodejs.org/）
- Git（安装：https://git-scm.com/）

### 🦀 后端启动

```bash
# 进入后端目录
cd rust-web-backend

# 构建项目
cargo build

# 运行服务器
cargo run
```

服务器将在 `http://localhost:8080` 启动

### 🌐 前端开发

```bash
# 进入前端目录
cd frontend

# 安装依赖
npm install

# 启动开发服务器
npm start
```

### 🤖 AI 工具配置

项目已配置 Model Context Protocol (MCP) 记忆服务器：

```bash
# 启动 MCP Memory 服务器
npm install -g @modelcontextprotocol/server-memory
npx @modelcontextprotocol/server-memory
```

## 🔧 功能特性

- ✅ **静态文件服务**: 后端自动为前端提供静态文件服务
- ✅ **模块化架构**: 清晰的代码结构和模块分离
- ✅ **AI 增强开发**: 集成 MCP 提供上下文记忆能力
- ✅ **现代工具链**: VS Code + Rust Analyzer + GitHub Copilot

## 📝 开发说明

### API 路由

- `GET /` - 前端主页
- `GET /static/*` - 静态资源文件

### 项目配置

- `Cargo.toml` - Rust 依赖和项目配置
- `package.json` - Node.js 依赖配置
- `.gitignore` - Git 忽略文件配置

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## 🙏 致谢

- [Rust](https://www.rust-lang.org/) - 系统编程语言
- [Actix Web](https://actix.rs/) - Rust Web 框架
- [Anthropic Claude](https://www.anthropic.com/) - AI 助手
- [Model Context Protocol](https://modelcontextprotocol.io/) - AI 上下文协议

---

⭐ 如果这个项目对您有帮助，请给个 Star！
