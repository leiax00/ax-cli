# ax-cli

> 个人开发环境配置管理工具。一个二进制 + 一份配置，多机同步，跨平台迁移。

## 设计理念

**一条 `ax` 命令，走到哪都能用。**

- **屏蔽终端差异**：bash、zsh、Windows PowerShell 5.1、PowerShell 7、cmd，用户只用 `ax`，不关心自己开的哪个终端
- **工具与配置分离**：ax 二进制纯工具，配置目录独立 git 仓库，可同步、迁移、分享
- **便携模式**：二进制旁放 `config/` 目录即可，适合 U盘、Windows 等场景
- **零运行时依赖**：Rust 单二进制，curl 下载即用，~4.6MB

## 架构

```
ax 二进制（纯工具，不含配置）
  +
配置仓库（~/.config/axconfig/，独立 git 仓库）
  =
完整的开发环境
```

## 快速开始

```bash
# 1. 下载二进制
curl -fLo ~/.local/bin/ax https://anyhub.yushe.ai/.../ax-linux-x86_64
chmod +x ~/.local/bin/ax

# 2. 安装（首次运行会自动初始化配置）
ax install

# 3. 重启终端
exec zsh
```

**不需要 clone 源码仓库**，配置模板全部内置在二进制里。

如果是源码方式更新补全与说明文案，执行：

```bash
cargo build --release
cp target/release/ax ~/.local/bin/ax
ax completion zsh
exec zsh
```

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

# 额外参数透传
ax add cc claude
cc --settings xxx
ax run cc -- --settings xxx

# SSH 连接管理
ax ssh add <别名> --host <主机> --user <用户> --auth key|password [--port 22] [--key <私钥>] [--password <密码>] [-d 描述]
ax ssh setup-key <别名> --host <主机> --user <用户> [--port 22] [--password <密码>] [--key <私钥>] [-d 描述]
ax ssh [别名]                    # 直接连接；省略别名时列出已保存连接
ax ssh list / ls                 # 列表
ax ssh rm / del <别名>           # 删除

# 系统管理
ax install                       # 安装 core 包 + 工具 + 配置部署
ax install --extras              # 额外安装开发增强包
ax push / pull                   # 配置同步快捷方式
ax proxy on/off/status           # 代理管理（加载 ax shell 配置后可直接生效）
ax proxy set-default <addr>      # 设置默认代理地址
ax completion bash/zsh/powershell # 安装 shell 补全
ax info                          # 查看当前配置
```

默认会把调用参数追加到原命令末尾；如果你需要自己控制插入位置，可以在命令内容里显式写 `$@`。

## Shell 体验

- `ax completion zsh` / `ax completion bash` 会安装对应 shell 的命令补全
- `ax install` 会自动刷新托管 shell 配置，并安装 zsh / bash 补全
- 补全由 `clap` 命令树自动生成，支持多层子命令，不需要手工维护层级
- zsh 下会显示命令说明；安装并加载 `zsh-autosuggestions` 后，还能得到基于历史命令的灰显预测

常见示例：

```bash
ax <Tab>
ax config <Tab>
ax env <Tab>
ax ssh <Tab>
ax env add --<Tab>
ax proxy <Tab>
```

如果补全没有立即生效，可手动执行：

```bash
source ~/.config/axconfig/bash/.zshrc
autoload -Uz compinit && compinit
```

如果使用 bash：

```bash
source ~/.config/axconfig/bash/.bashrc
```

## 语言与本地化

- `ax --help` 与 shell 补全说明支持中文和英文
- 语言选择顺序：`AX_LANG` > `LC_ALL` > `LC_MESSAGES` > `LANG` > 默认中文
- 当前支持 `zh` / `en`

示例：

```bash
ax --help
AX_LANG=en ax --help
AX_LANG=en ax completion zsh
```

## 配置目录结构

```
~/.config/axconfig/            # 配置根目录（XDG 标准）
├── config.yaml                # 主配置
├── config.d/                  # 分块配置（自动合并）
│   ├── commands.yaml          # 用户动态命令
│   ├── env.yaml               # 用户动态环境变量
│   └── ssh.yaml               # 用户动态 SSH 连接
├── bash/.bashrc               # bash 配置片段（由 ~/.bashrc 引入）
├── bash/.zshrc                # zsh 配置片段（由 ~/.zshrc 引入）
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
    > ~/.config/axconfig/（默认）
```

Windows 便携式：把 `ax.exe` 和 `config/` 放同一目录即可。

## 支持的发行版

| 发行版 | 包管理器 | 包列表 |
|--------|---------|--------|
| Ubuntu / Debian / Linux Mint | apt | `packages/ubuntu.txt` |
| Fedora / RHEL / CentOS | dnf | `packages/fedora.txt` |
| Arch / Manjaro | pacman | `packages/arch.txt` |

自动检测系统，无需手动选择。
包作用说明见 `docs/packages.md`。

## Feature 支持矩阵

说明：

- `✅`：当前已支持
- `⚠️`：部分支持，通常需要手动接入或体验有差异
- `❌`：当前未提供

### 核心 CLI 能力

这些能力和交互 shell 基本无关，进入任意终端后直接执行 `ax ...` 即可：

| 能力点 | zsh | bash | Windows PowerShell 5.1 | PowerShell 7 | cmd | 说明 |
|------|------|------|------|------|------|------|
| CLI 基础命令解析与执行 | ✅ | ✅ | ✅ | ✅ | ✅ | Rust 单二进制 |
| 配置初始化与路径管理 | ✅ | ✅ | ✅ | ✅ | ✅ | `ax config init/path/remote` |
| 配置同步 | ✅ | ✅ | ✅ | ✅ | ✅ | `ax config push/pull` |
| 配置导入导出 | ✅ | ✅ | ✅ | ✅ | ✅ | `ax config export/import` |
| 自定义命令 CRUD | ✅ | ✅ | ✅ | ✅ | ✅ | `ax add/edit/rm/list` |
| 自定义命令执行 | ✅ | ✅ | ✅ | ✅ | ✅ | `ax run [name]` |
| SSH 连接管理 | ✅ | ✅ | ✅ | ✅ | ✅ | `ax ssh add/list/rm` + `ax ssh <name>` |
| 环境变量管理 | ✅ | ✅ | ✅ | ✅ | ✅ | `ax env add/edit/show/pause/resume/tags` |
| 代理状态查看 | ✅ | ✅ | ✅ | ✅ | ✅ | `ax proxy status` |
| Help 多语言 | ✅ | ✅ | ✅ | ✅ | ✅ | `AX_LANG` / `LANG` 驱动 |
| WezTerm 配置模板 | ✅ | ✅ | ✅ | ✅ | ✅ | 配置文件可管理，是否使用取决于用户终端 |
| Starship 安装 | ✅ | ✅ | ⚠️ | ⚠️ | ⚠️ | 工具可安装，但 shell 初始化脚本目前只托管 bash/zsh |
| fzf 安装 | ✅ | ✅ | ⚠️ | ⚠️ | ❌ | 当前仓库主要提供 bash/zsh 集成 |
| CI/CD 构建发布 | ✅ | ✅ | ✅ | ✅ | ✅ | 与交互 shell 无关 |
| Git 配置模板 | ⚠️ | ⚠️ | ⚠️ | ⚠️ | ⚠️ | 模板已预留，内容需用户补充 |
| tmux 配置模板 | ⚠️ | ⚠️ | ⚠️ | ⚠️ | ⚠️ | 模板已预留 |

### Shell / 终端集成能力

这些能力依赖各 shell 的补全系统、启动脚本或插件机制，不同终端体验不同：

| 能力点 | zsh | bash | Windows PowerShell 5.1 | PowerShell 7 | cmd | 说明 |
|------|------|------|------|------|------|------|
| 托管 shell 配置自动接入 | ✅ | ✅ | ❌ | ❌ | ❌ | 当前只维护 `bash/.zshrc` 和 `bash/.bashrc` |
| `ax completion <shell>` 生成补全脚本 | ✅ | ✅ | ✅ | ✅ | ❌ | `powershell` 默认覆盖 5.1 和 7；`pwsh` 仅覆盖 7 |
| `ax install` 自动安装补全 | ✅ | ✅ | ❌ | ❌ | ❌ | 当前安装流程只自动装 zsh/bash 补全 |
| 多层子命令补全 | ✅ | ✅ | ⚠️ | ⚠️ | ❌ | PowerShell 可生成补全，但未自动接入 |
| 补全候选描述显示 | ✅ | ❌ | ❌ | ❌ | ❌ | 目前只有 zsh 展示命令说明 |
| 历史灰显预测 | ✅ | ❌ | ❌ | ❌ | ❌ | 依赖 `zsh-autosuggestions` |
| 语法高亮 | ✅ | ❌ | ❌ | ❌ | ❌ | 依赖 `zsh-syntax-highlighting` |
| `ax proxy on/off` 直接修改当前 shell 环境 | ✅ | ✅ | ⚠️ | ⚠️ | ⚠️ | zsh/bash 通过托管函数直接 `eval`；PowerShell/cmd 需手动执行提示命令 |
| `ax env load` 输出当前 shell 可执行脚本 | ✅ | ✅ | ✅ | ✅ | ✅ | 自动按 shell 输出 `export` / `$env:` / `set` |
| `ax ssh <name>` 调起系统 SSH | ✅ | ✅ | ✅ | ✅ | ✅ | password 模式优先用 `sshpass`，不可用时打印密码后退化为普通 `ssh` |
| 自定义命令快捷函数 | ✅ | ✅ | ❌ | ❌ | ❌ | `commands.sh` 只为 bash/zsh 生成并 source |
| 重新加载托管配置即可生效 | ✅ | ✅ | ❌ | ❌ | ❌ | `source ~/.config/axconfig/bash/.zshrc/.bashrc` |

## 编译

```bash
cargo build --release
# 产物: target/release/ax (~4.6MB)
```

## 仓库结构

```
ax-cli/
├── Cargo.toml                  # Rust 项目配置
├── src/                        # Rust 源码
│   ├── main.rs                 # 入口
│   ├── cli.rs                  # 命令定义（clap）
│   ├── config.rs               # 配置加载/保存/模板
│   ├── detect.rs               # 系统检测
│   ├── packages.rs             # 包管理
│   ├── shell.rs                # zsh + 插件
│   ├── tools.rs                # fzf/starship/字体
│   └── commands/               # 各子命令实现
├── config/                     # 配置模板（ax config init 使用）
│   ├── bash/
│   ├── wezterm/
│   ├── packages/
│   └── git/
├── install/                    # 安装脚本
│   ├── install-ax.sh
│   ├── install-ax.ps1
│   └── npm/
├── .github/workflows/          # CI/CD
└── docs/                       # 文档
```

## 许可

个人使用。
