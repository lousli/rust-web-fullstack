# 医生数据分析系统

> 一个基于 Rust + Web 技术栈的医生数据分析和管理系统

## 🚀 项目概述

本项目是一个现代化的医生数据分析系统，用于管理医生信息、配置分析权重、生成数据报表和可视化展示。系统采用前后端分离架构，后端使用 Rust + Actix Web，前端使用原生 Web 技术。

## 📁 项目结构

```
医生数据分析系统/
├── 📖 docs/                    # 项目文档
│   ├── 📋 项目概述.md           # 项目整体说明
│   ├── 🔧 开发指南/             # 开发相关文档
│   ├── 📚 API文档/              # API 接口文档
│   └── 🚀 部署指南/             # 部署相关文档
├── 🦀 rust-web-backend/        # Rust 后端服务
│   ├── src/                    # 源代码
│   │   ├── main.rs            # 应用入口
│   │   ├── handlers/          # 请求处理器
│   │   ├── models/            # 数据模型
│   │   └── routes/            # 路由定义
│   ├── frontend/              # 内嵌前端资源
│   └── Cargo.toml             # Rust 项目配置
├── 🌐 frontend/                # 独立前端项目
│   ├── src/                   # 前端源码
│   └── package.json           # Node.js 配置
└── 🛠️ tools/                   # 开发工具和脚本
```

## 🛠️ 技术栈

### 后端技术
- **语言**: Rust 2021 Edition
- **Web框架**: Actix Web 4.0
- **数据库**: SQLite (开发) / PostgreSQL (生产)
- **序列化**: Serde
- **异步运行时**: Tokio

### 前端技术
- **基础**: HTML5 + CSS3 + ES6+ JavaScript
- **图表**: Chart.js
- **样式**: 原生 CSS + CSS Grid/Flexbox
- **图标**: Font Awesome

### 开发工具
- **AI辅助**: Model Context Protocol (MCP) + Claude
- **代码补全**: GitHub Copilot
- **开发环境**: VS Code
- **版本控制**: Git

## 🚀 快速开始

### 环境要求

- Rust 1.70+ ([安装指南](https://rustup.rs/))
- Node.js 16+ (仅用于前端开发服务器)
- Git

### 运行后端服务

```powershell
# 进入后端目录
cd rust-web-backend

# 安装依赖并运行
cargo run
```

服务将在 `http://localhost:8080` 启动

### 运行前端开发服务器

```powershell
# 进入前端目录
cd frontend

# 安装依赖
npm install

# 启动开发服务器
npm start
```

前端开发服务器将在 `http://localhost:8081` 启动

## 🔧 核心功能

- ✅ **医生信息管理** - CRUD 操作、批量导入、数据验证
- ✅ **权重配置管理** - 评分指标权重设置、历史记录
- ✅ **数据分析展示** - 多维度评分、可视化图表
- ✅ **报表导出功能** - Excel/CSV 导出、自定义报表
- 🚧 **用户权限管理** - 角色权限、操作审计 (开发中)

## 📚 文档导航

- [项目概述](./docs/项目概述.md) - 详细的项目背景和架构说明
- [开发指南](./docs/开发指南/) - 开发环境搭建和代码规范
- [API 文档](./docs/API文档/) - 后端 API 接口文档
- [部署指南](./docs/部署指南/) - 生产环境部署说明

## 🤝 贡献指南

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 打开 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## 🙏 致谢

- [Actix Web](https://actix.rs/) - 高性能的 Rust Web 框架
- [Chart.js](https://www.chartjs.org/) - 优秀的图表库
- [Font Awesome](https://fontawesome.com/) - 丰富的图标库

---

⭐ 如果这个项目对您有帮助，请给一个 Star！
