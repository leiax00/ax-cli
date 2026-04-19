# ax-system-basic

> 个人开发环境配置管理工具。一个二进制 + 一份配置，多机同步，跨平台迁移。

## 设计理念

**一条 `ax` 命令，走到哪都能用。**

- **屏蔽终端差异**：bash、zsh、PowerShell 5/7、cmd，用户只用 `ax`，不关心自己开的哪个终端
- **工具与配置分离**：ax 二进制纯工具，配置目录独立 git 仓库，可同步、迁移、分享
- **便携模式**：二进制旁放 `config/` 目录即可，适合 U盘、Windows 等场景
- **零运行时依赖**：Rust 单二进制，curl 下载即用，~4.6MB

## 架构

```
ax 二进制（纯工具，不含配置）
  +
配置仓库（~/.config/ax-cli/，独立 git 仓库）
  =
完整的开发环境
```

## 快速开始

```bash
# 1. 下载二进制
curl -fLo ~/.local/bin/ax https://anyhub.yushe.ai/.../ax-linux-x86_64
chmod +x ~/.local/bin/ax

# 2. 初始化配置（自动生成默认配置 + git init）
ax config init

# 3. 安装
ax install

# 4. 重启终端
exec zsh
```

**不需要 clone 源码仓库**，配置模板全部内置在二进制里。

## 命令总览

```bash
# 配置管理
ax config init [--force]          # 初始化配置目录 + git repo
ax config remote <url>            # 设置/查看远程仓库
ax config push                    # 推送配置
ax config pull                    # 拉取配置
ax config export [-f]             # 导出为 tar.gz（-f 含二进制）
ax config import <file>           # 导入配置
ax config path                    # 显示配置目录

# 环境变量管理
ax env add <名> <值> [-d 描述] [-t 标签]   # 添加
ax env edit <名> [-v 值] [-d 描述] [-t 标签] # 修改
ax env rm <名>...                           # 删除
ax env show [--all] [-t 标签]               # 列表
ax env pause <名> / -t <标签> / --all       # 暂停
ax env resume <名> / -t <标签> / --all      # 恢复
ax env load                                # 输出 shell exports
ax env tags                                # 查看标签

# 命令管理
ax add <名> <命令> [描述]        # 添加
ax edit <名>                     # 编辑
ax list / ls                     # 列表
ax rm / del <名>                 # 删除
ax run [名]                      # 执行
ax <名>                          # 快捷执行

# 系统管理
ax install                       # 一键安装（包+工具+配置部署）
ax push / pull                   # 配置同步快捷方式
ax proxy on/off/status           # 代理管理
ax completion bash/zsh/powershell # 安装 shell 补全
ax info                          # 查看当前配置
```

## 配置目录结构

```
~/.config/ax-cli/              # 配置根目录（XDG 标准）
├── config.yaml                # 主配置
├── config.d/                  # 分块配置（自动合并）
│   ├── commands.yaml          # 用户动态命令
│   └── env.yaml               # 用户动态环境变量
├── bash/.zshrc                # shell 配置
├── wezterm/wezterm.lua        # 终端配置
├── packages/                  # 包列表
│   └── ubuntu.txt
├── git/.gitconfig             # git 配置
└── .git/                      # 配置仓库（可选关联远程）
```

### 配置优先级

```
AX_CONFIG_DIR 环境变量（最高）
  > 可执行文件同级 config/ 或 config.yaml（便携模式）
    > ~/.config/ax-cli/（默认）
```

Windows 便携式：把 `ax.exe` 和 `config/` 放同一目录即可。

## 支持的发行版

| 发行版 | 包管理器 | 包列表 |
|--------|---------|--------|
| Ubuntu / Debian / Linux Mint | apt | `packages/ubuntu.txt` |
| Fedora / RHEL / CentOS | dnf | `packages/fedora.txt` |
| Arch / Manjaro | pacman | `packages/arch.txt` |

自动检测系统，无需手动选择。

## 实现状态

| 模块 | 状态 | 说明 |
|------|------|------|
| ax CLI (Rust) | ✅ | 单二进制零依赖，5 平台编译 |
| 配置管理 | ✅ | YAML，级联合并，独立 git 仓库 |
| 命令管理 | ✅ | CRUD + 自动同步 + shell 补全 |
| 环境变量管理 | ✅ | 标签分组，暂停/恢复，eval 加载 |
| 代理管理 | ✅ | eval $(ax proxy on) |
| Shell 补全 | ✅ | bash/zsh/powershell |
| 配置导入导出 | ✅ | 跨机器迁移，含二进制的便携包 |
| WezTerm 配置 | ✅ | Catppuccin Mocha，tmux 风格快捷键 |
| zsh 配置 | ✅ | 自动建议、语法高亮、增强补全 |
| Starship Prompt | ✅ | 通过 install.sh 安装 |
| fzf 模糊搜索 | ✅ | 通过 install.sh 安装 |
| CI/CD | ✅ | GitHub Actions + Gitea Actions |
| Git 配置 | 🔲 | 需要填充 .gitconfig |
| tmux 配置 | 🔲 | 预留 |

## 编译

```bash
cd bin/ax-rs
cargo build --release
# 产物: target/release/ax (~4.6MB)
```

## 仓库结构

```
ax-system-basic/
├── bin/ax-rs/                 # Rust 源码
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs
│       ├── cli.rs             # 命令定义
│       ├── config.rs          # 配置加载/保存
│       ├── detect.rs          # 系统检测
│       ├── commands/          # 各子命令实现
│       ├── packages.rs
│       ├── shell.rs
│       └── tools.rs
├── install.sh                 # 一键部署（下载二进制+安装包+部署配置）
├── .github/workflows/         # CI/CD
├── .gitea/workflows/
└── docs/                      # 文档
```

## 详细文档

详见 [`docs/`](./docs/) 目录。

## 许可

个人使用。
