# 技术选型

## 操作系统：Ubuntu 24.04 LTS

| 需求 | Ubuntu 优势 |
|------|------------|
| 国内软件 | 微信/QQ/WPS 有原生 deb 或 flatpak，适配最成熟 |
| 嵌入式开发 | ESP-IDF 官方推荐，串口工具/烧录工具 apt 直装 |
| 云/C端开发 | Docker、Node.js、各种 SDK 开箱即用 |
| AI coding | Cursor/Windsurf 有 Linux 原生版，终端工具原生支持 |
| 稳定性 | LTS 支持 5 年，2029 年前不用操心升级 |

### 备选对比

- **Deepin**：国产应用好，但稳定性和社区差
- **Arch/Manjaro**：AUR 全，但滚动更新不稳定
- **Fedora**：优秀，但嵌入式和国内软件兼容性不如 Ubuntu

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

## 自定义命令管理：ax

自己写的轻量 CLI，解决 alias 的问题：

| alias 的痛点 | ax 的方案 |
|-------------|----------|
| 无法查看已有命令 | `ax list` 带描述列表 |
| 无法搜索 | `ax` 不带参数 → fzf 交互选择 |
| 无法 Tab 补全 | bash/zsh 补全脚本 |
| 多机不同步 | 自动 commit + push 到 git |

---

**下一步** → [模块详细文档](./modules/README.md)
