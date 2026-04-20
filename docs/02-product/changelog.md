# 变更日志

格式遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)。

## [Unreleased]

### 新增
- 文档体系建立（产品需求、技术设计、架构文档）
- 基于 `clap` 的多层 shell 补全，支持 zsh 命令说明展示
- help 与补全说明支持中文 / 英文自动切换

### 变更
- 重命名为 ax-cli，配置目录改为 axconfig
- 迁移为根级 Rust 项目，移除旧 shell 架构
- `ax install` 自动刷新托管 shell 配置并安装 bash / zsh 补全
