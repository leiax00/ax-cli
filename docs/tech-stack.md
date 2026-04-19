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

通过自动检测系统，选择对应包管理器和包列表，核心配置跨发行版通用。

## 终端：WezTerm

| 特性 | 说明 |
|------|------|
| 性能 | Rust 编写，GPU 加速（WebGpu），120fps |
| 分屏/标签 | 内置，不需要 tmux 也能用 |
| 配置 | Lua，灵活可编程 |
| 跨平台 | Linux/macOS/Windows 一套配置 |
| tmux 兼容 | Ctrl+A Leader 键，SSH 时无缝切 tmux |

## Shell：zsh

| 特性 | 说明 |
|------|------|
| 插件生态 | autosuggestions、syntax-highlighting、completions |
| 社区支持 | 新工具优先支持 zsh |
| 通配符 | `**/*.js` 递归匹配原生支持 |
| 历史管理 | 去重、共享、实时追加 |

### 不选 Oh My Zsh

太重（0.3s+ 启动），手动装 3 个插件更干净。

## Prompt：Starship

跨 shell，显示 git 状态、语言版本、命令耗时，Rust 编写毫秒级渲染。

## 模糊搜索：fzf

命令历史搜索（Ctrl+R）、文件/目录快速跳转。

## CLI 工具：ax（Rust）

自己写的开发环境管理工具，解决以下问题：

| 问题 | ax 的方案 |
|------|----------|
| alias 无法查看/搜索/补全 | `ax list` + shell 补全 |
| 环境变量散落各处 | `ax env` 统一管理 + 标签 + 暂停 |
| 配置不能同步 | 配置目录即 git 仓库，`ax push/pull` |
| 新机器部署麻烦 | `ax config init && ax install` |
| 迁移困难 | `ax config export/import` |
| 代理管理繁琐 | `eval $(ax proxy on)` |

### 为什么用 Rust

| 优势 | 说明 |
|------|------|
| 单二进制 | 无运行时依赖，curl 下载即用 |
| 跨平台 | Linux/macOS/Windows 同源编译 |
| 性能 | 毫秒级启动 |
| 便携模式 | 二进制旁放 config/ 即可，适合 Windows/U盘 |

## 架构：工具与配置分离

```
ax 二进制                    配置仓库
┌──────────────┐            ┌──────────────────┐
│  纯 CLI 工具  │    加载     │  config.yaml      │
│  零配置内置   │ ────────→  │  config.d/*.yaml  │
│  ~4.6MB      │            │  bash/.zshrc      │
└──────────────┘            │  wezterm/...      │
                            │  packages/        │
                            └────────┬─────────┘
                                     │ git push/pull
                                     ▼
                              远程仓库（可选）
```

### 配置优先级（从高到低）

```
AX_CONFIG_DIR 环境变量
  > 可执行文件同级 config/ 或 config.yaml
    > ~/.config/ax-cli/
```

---

**下一步** → [模块详细文档](./modules/README.md)
