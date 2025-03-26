# Office Tools Rust 🚀

一个基于 Rust 后端 + Web 前端的本地办公自动化工具，用于执行和管理 Python 脚本。

## 项目概述 📋

本项目旨在提供一个轻量级的 Web 界面，用于执行和监控 Python 脚本。通过浏览器即可方便地运行脚本并实时查看输出结果。

### 核心功能 ⭐

- 🌐 Web 界面操作 Python 脚本
- 📊 实时显示脚本执行输出
- 🔄 支持多脚本管理
- 🛡️ 安全的脚本执行环境
- 📱 响应式设计，支持移动端

## 技术栈 🛠️

- **后端**：Rust + Actix-web
- **前端**：HTML + JavaScript (可升级至 Vue/React)
- **脚本**：Python

## 系统架构 🏗️

```
[ Browser ] <-> [ Rust Server (Actix) ] <-> [ Python Scripts ]
     ↑                    ↑                        ↑
     |                    |                        |
  用户交互          HTTP/WebSocket 服务         脚本执行层
```

## 开发路线图 📈

### 第一阶段 - 基础功能 ✅
- [x] 项目基础架构搭建
- [ ] 简单的 Web 界面
- [ ] Python 脚本执行功能
- [ ] 基本输出展示

### 第二阶段 - 功能增强 🔄
- [ ] WebSocket 实时输出
- [ ] 多脚本支持
- [ ] 参数传递功能
- [ ] 执行状态管理

### 第三阶段 - 安全与优化 🔐
- [ ] 用户认证
- [ ] 脚本安全控制
- [ ] 性能优化
- [ ] 错误处理完善

## 项目结构 📁

```
office-tools-rust/
├── src/                # Rust 源代码
├── static/            # 静态文件
├── templates/         # HTML 模板
├── scripts/          # Python 脚本
├── docs/             # 文档
└── tests/            # 测试文件
```

## 快速开始 🚀

### 环境要求
- Rust 1.5+
- Python 3.7+
- Node.js 14+ (可选，用于前端开发)

### 安装步骤
1. 克隆仓库
```bash
git clone [repository-url]
cd office-tools-rust
```

2. 安装依赖
```bash
cargo build
```

3. 运行服务
```bash
cargo run
```

4. 访问服务
打开浏览器访问 `http://localhost:8080`

## 贡献指南 👥

欢迎提交 Issue 和 Pull Request！

## 许可证 📄

MIT License 