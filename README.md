# ax-system-basic

> 个人开发环境配置，一键部署，多机同步，多发行版支持。

## 快速开始

```bash
git clone https://anyhub.yushe.ai/leiax00/ax-system-basic.git ~/.ax
~/.ax/install.sh
exec zsh
```

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
| ax 命令管理器 | ✅ | Rust 重写，单二进制，零依赖，多平台 |
| WezTerm 配置 | ✅ | Catppuccin Mocha，tmux 风格快捷键 |
| zsh 配置 | ✅ | 自动建议、语法高亮、增强补全 |
| Starship Prompt | ✅ | 通过 install.sh 安装 |
| fzf 模糊搜索 | ✅ | 通过 install.sh 安装 |
| Proxy 管理 | ✅ | pn/pf/ps 快捷启停代理 |
| 一键部署 | ✅ | install.sh，模块化，多发行版支持 |
| ax update | ✅ | 自动拉取、检查包、更新插件和字体 |
| 命令库远程同步 | ✅ | add/edit/rm 后自动 commit + push |
| Git 配置 | 🔲 | 需要填充 .gitconfig |
| tmux 配置 | 🔲 | 预留，后续需要时添加 |
| Neovim / IDE 配置 | 🔲 | 按需添加 |

## 仓库结构

```
ax-system-basic/
├── install.sh              # 一键部署入口
├── ax-commands.json        # 自定义命令库（自动同步）
├── bin/
│   └── ax                  # 命令管理器 + ax update
├── lib/                    # 模块库（install.sh 和 ax update 共用）
│   ├── detect.sh           # 系统检测 + 包管理器选择
│   ├── packages.sh         # 系统包安装/检查
│   ├── shell.sh            # zsh + 插件安装/更新
│   ├── tools.sh            # fzf / starship / 字体
│   └── deploy.sh           # 配置文件链接/备份
├── packages/               # 各发行版包列表
│   ├── ubuntu.txt
│   ├── fedora.txt
│   └── arch.txt
├── bash/
│   ├── .zshrc              # zsh 配置
│   ├── .bashrc             # bash 配置（兼容）
│   ├── proxy.sh            # 代理启停
│   └── completions/
│       ├── ax              # bash 补全
│       └── ax.zsh          # zsh 补全
├── wezterm/
│   └── wezterm.lua         # 终端配置
├── git/
│   └── .gitconfig          # git 配置（待填充）
└── docs/
    └── ...                 # 详细文档
```

## 详细文档

详见 [`docs/`](./docs/) 目录，从 [需求总览](./docs/README.md) 开始。

## 许可

个人使用。
