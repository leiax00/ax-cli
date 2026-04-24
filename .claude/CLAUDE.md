# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## 项目概述

个人开发环境 CLI 管理工具（Rust），单二进制零运行时依赖，配置与工具分离，支持多机同步和跨平台迁移。

## 关键命令

```bash
# 编译
cargo build --release    # 产物: target/release/ax

# 常用 CLI
ax config init [--force]       # 初始化配置目录 + git repo
ax install                     # 一键安装（包+工具+配置部署）
ax push / pull                 # 配置同步
ax proxy on/off/status         # 代理管理
ax info                        # 查看当前配置
```

无测试框架。验证方式：编译后在目标环境运行 `ax install`。

## 架构

### 源码结构

```
src/
├── main.rs          # 入口，解析 CLI 路由到各命令
├── cli.rs           # clap 命令定义（Commands, ConfigAction, EnvAction, ProxyAction）
├── config.rs        # YAML 配置加载/保存、多源级联合并、模板生成
├── detect.rs        # OS 检测 + 包管理器选择 + packages/{os}.txt 路径
├── packages.rs      # 系统包安装（读包列表，跳过已安装）
├── shell.rs         # zsh 安装/设默认 + 插件管理
├── tools.rs         # fzf / starship / Nerd Font 安装
└── commands/        # 各子命令实现
    ├── add.rs, edit.rs, rm.rs, list.rs, run.rs   # 命令 CRUD
    ├── config.rs    # 配置 init/remote/push/pull/export/import/path
    ├── env.rs       # 环境变量管理（标签分组、暂停/恢复、eval 加载）
    ├── proxy.rs     # 代理 on/off/status
    ├── install.rs   # 完整安装流程（包+工具+配置部署+备份）
    ├── push.rs, pull.rs   # 配置同步
    ├── completion.rs      # shell 补全生成（bash/zsh/powershell）
    └── info.rs      # 配置信息展示
```

### 配置系统

配置目录默认 `~/.config/ax-cli/`，优先级：`AX_CONFIG_DIR` 环境变量 > 可执行文件同级 `config/`（便携模式）> 默认路径。

`config.yaml` 定义部署链接、包列表路径等；`config.d/` 下 `commands.yaml` 和 `env.yaml` 存储用户动态数据。配置支持深合并和环境变量替换。

`config.rs` 通过 `include_str!` 从 `config/` 目录嵌入 shell/wezterm/tmux 模板（`TEMPLATE_ZSHRC`、`TEMPLATE_BASHRC`、`TEMPLATE_WEZTERM`、`TEMPLATE_TMUX`），`TEMPLATE_CONFIG_YAML` 为代码内嵌。`ax config init` 时写入用户配置目录。

### 多发行版支持

`detect.rs` 检测 OS ID，映射到 `packages/{os_id}.txt`。新增发行版：加包列表文件即可，detect.rs 已有 fallback 逻辑。

### 仓库中的非代码文件

- `config/`：配置模板（bash/.zshrc、wezterm/wezterm.lua、packages/、git/），供参考和 `ax config init` 使用
- `install/`：安装脚本（install-ax.sh、install-ax.ps1、npm/）
- `.github/workflows/`：CI/CD（build + release，5 平台交叉编译）

## 开发约定

- Rust edition 2021，release profile 使用 `opt-level = "s"` + `lstrip`
- CLI 框架：clap derive 模式，命令定义集中在 `cli.rs`
- 配置格式：YAML（serde_yaml）
- 新增命令：在 `cli.rs` 加枚举变体 → `commands/` 加实现文件 → `main.rs` 路由

## 文档体系（强制规则）

完整文档位于 `docs/`，采用渐进式披露组织。

**修改任何模块前，必须先阅读对应的设计文档：**

| 修改范围 | 必读文档 |
|---------|---------|
| CLI 命令 | `docs/03-design/command-system.md` |
| 配置系统 | `docs/03-design/config-system.md` |
| 安装流程 | `docs/03-design/install-flow.md` |
| 环境变量 | `docs/03-design/env-management.md` |
| 代理功能 | `docs/03-design/proxy-system.md` |
| 平台支持 | `docs/03-design/platform-support.md` |
| 包列表 | `docs/03-design/packages.md` |
| 终端快捷键 | `docs/03-design/terminal-keybindings.md` |
| 模块依赖 | `docs/04-architecture/module-dependencies.md` |
| 新增功能 | `docs/04-architecture/extension-guide.md` |

**修改代码后，必须同步更新受影响的文档。** 如果代码行为与文档不一致，更新文档使其反映实际实现。详细规则见 `docs/01-standards/doc-first-rule.md`。
