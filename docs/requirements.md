# 需求总览

## 背景

个人开发主力机，需要一套可快速迁移的开发环境配置。

## 使用场景

### 日常开发
- 云端产品开发（后端服务、API、数据库）
- C 端产品开发（Web 前端、App）
- AI 辅助编码（Cursor、Claude Code、Codex CLI 等）

### 嵌入式开发
- ESP-IDF 开发套件
- 串口通信（CH340 / CP2102）
- 固件烧录（dfu-util、esptool、openocd）

### 国内软件
- 微信（Linux 原生版）
- QQ（Linux 原生版）
- WPS Office（Linux 原生版）

## 核心需求

### 1. 环境可迁移
- 所有配置集中管理，一个 git 仓库搞定
- 新机器 clone → install.sh → 完事
- 自定义命令自动同步到远程

### 2. 终端体验
- 高性能终端（GPU 渲染）
- 智能补全（历史命令提示、语法高亮）
- 自定义命令管理（ax）
- tmux 风格快捷键，SSH 时无缝切换

### 3. 开发效率
- 模糊搜索（fzf）
- 漂亮的 prompt（Starship）
- 常用工具开箱即用（ripgrep、fd、bat 等）

### 4. 嵌入式友好
- 串口工具一条命令装好
- ESP-IDF 官方支持的平台
- USB 转串口驱动免装

## 约束

- 操作系统：Ubuntu 24.04 LTS
- 配置管理：符号链接 + git
- 不用 Oh My Zsh（太重，手动管理插件）
- 不用 nvm/pyenv 等版本管理器（apt 足够，按需后续添加）

---

**下一步** → [技术选型](./tech-stack.md)
