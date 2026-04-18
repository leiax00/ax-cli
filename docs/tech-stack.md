# 技术选型

## 操作系统

### 主力：Ubuntu 24.04 LTS

| 需求 | Ubuntu 优势 |
|------|------------|
| 国内软件 | 微信/QQ/WPS 有原生 deb 或 flatpak，适配最成熟 |
| 嵌入式开发 | ESP-IDF 官方推荐，串口工具/烧录工具 apt 直装 |
| 云/C端开发 | Docker、Node.js、各种 SDK 开箱即用 |
| AI coding | Cursor/Windsurf 有 Linux 原生版，终端工具原生支持 |
| 稳定性 | LTS 支持 5 年，2029 年前不用操心升级 |

### 兼容：Fedora / Arch

通过 `lib/detect.sh` 自动检测系统，选择对应包管理器和包列表，核心配置（zsh、WezTerm、ax）跨发行版通用。

| 发行版 | 包管理器 | 包列表文件 |
|--------|---------|-----------|
| Ubuntu / Debian / Linux Mint | apt | `packages/ubuntu.txt` |
| Fedora / RHEL / CentOS | dnf | `packages/fedora.txt` |
| Arch / Manjaro | pacman | `packages/arch.txt` |

## 终端：WezTerm

| 特性 | 说明 |
|------|------|
| 性能 | Rust 编写，GPU 加速（WebGpu），120fps |
| 分屏/标签 | 内置，不需要 tmux 也能用 |
| 配置 | Lua，灵活可编程 |
| 跨平台 | Linux/macOS/Windows 一套配置 |
| tmux 兼容 | Ctrl+A Leader 键，SSH 时无缝切 tmux |

### 备选对比

- **Alacritty**：极简高性能，但功能太朴素，无分屏
- **Kitty**：不错，但配置不如 WezTerm 灵活
- **GNOME Terminal**：默认够用但不好用

## Shell：zsh

| 特性 | 说明 |
|------|------|
| 插件生态 | autosuggestions、syntax-highlighting、completions |
| 社区支持 | 新工具优先支持 zsh |
| 通配符 | `**/*.js` 递归匹配原生支持 |
| 历史管理 | 去重、共享、实时追加 |

### 不选 Oh My Zsh

- 太重，启动慢（0.3s+）
- 更新容易出问题
- 手动装 3 个插件更干净

## Prompt：Starship

- 跨 shell（zsh/bash/fish）
- 显示 git 状态、语言版本、命令耗时
- Rust 编写，毫秒级渲染

## 模糊搜索：fzf

- 命令历史搜索（Ctrl+R）
- 文件/目录快速跳转
- 与 ax 命令管理器集成

## 代理管理：proxy.sh

- `proxy_on [地址]` / `proxy_off` / `proxy_status`
- 短别名 `pn` / `pf` / `ps`
- 自动设置 http/https/all_proxy + no_proxy

## 自定义命令管理：ax

自己写的轻量 CLI，解决 alias 的问题：

| alias 的痛点 | ax 的方案 |
|-------------|----------|
| 无法查看已有命令 | `ax list` 带描述列表 |
| 无法搜索 | `ax` 不带参数 → fzf 交互选择 |
| 无法 Tab 补全 | bash/zsh 补全脚本 |
| 多机不同步 | 自动 commit + push 到 git |
| 环境更新麻烦 | `ax update` 一键更新所有组件 |

## 架构：模块化

`lib/` 目录下每个脚本是一个独立模块，`install.sh` 和 `ax update` 共用同一套模块：

| 模块 | 职责 |
|------|------|
| `lib/detect.sh` | 检测系统 + 选择包管理器 |
| `lib/packages.sh` | 系统包安装/检查 |
| `lib/shell.sh` | zsh + 插件安装/更新 |
| `lib/tools.sh` | fzf / starship / 字体 |
| `lib/deploy.sh` | 配置文件链接/备份 |

新增发行版：加 `packages/xxx.txt` + `detect.sh` 加 case
新增组件：加 `lib/xxx.sh` + `install.sh` 里 source 调用

---

**下一步** → [模块详细文档](./modules/README.md)
