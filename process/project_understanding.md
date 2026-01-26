# 项目理解报告：MicroBin

## 1. 项目概览
MicroBin 是一个自托管、轻量级且功能丰富的 Pastebin 和 URL 缩短服务应用。
它旨在提供简单、高性能的文件和文本分享解决方案，支持单文件部署。

## 2. 技术栈
- **核心语言**: Rust (Edition 2021)
- **Web 框架**: Actix-Web v4 (高性能异步 Web 框架)
- **模板引擎**: Askama (编译时检查的 Jinja 风格模板)
- **数据存储**:
  - SQLite (可选依赖 `rusqlite`)
  - JSON 文件系统存储
- **前端技术**: 原生 HTML/JS, 极简 CSS (Water.css)
- **其他关键库**:
  - `syntect`: 代码语法高亮
  - `magic-crypt`: 加密支持
  - `qrcode-generator`: 二维码生成
  - `actix-multipart`: 文件上传处理

## 3. 目录与架构分析
- **src/main.rs**: 程序入口，负责启动 Actix 服务器，加载配置。
- **src/endpoints/**: 包含各个路由的处理逻辑（如上传、查看、API）。
- **src/pasta.rs**: 核心数据模型（猜测 "Pasta" 是对 Paste/上传内容的内部命名），定义了数据结构和操作。
- **src/util/**: 通用工具函数模块。
- **templates/**: 存放 Askama HTML 模板文件，用于服务器端渲染。
- **Dockerfile & compose.yaml**: 容器化部署配置。
- **.env**: 环境变量配置（示例）。

## 4. 核心功能
- **文本/代码分享**: 支持语法高亮。
- **文件上传**: 支持大文件和多媒体。
- **URL 缩短**: 提供重定向服务。
- **隐私与安全**:
  - 客户端/服务端加密。
  - 私有/公开上传控制。
  - 自动过期设置。
- **部署便捷性**: 生成单一可执行文件，也支持 Docker 一键部署。

## 5. 当前状态
项目处于活跃开发状态（Cargo 版本为 2.1.0），依赖项较新，代码结构清晰。
process 目录下已有执行日志，表明符合当前的开发规范。
