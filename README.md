# ax-system-basic

> 个人开发环境配置，一键部署，多机同步。

## 快速开始

```bash
git clone https://anyhub.yushe.ai/leiax00/ax-system-basic.git ~/.dotfiles
~/.dotfiles/install.sh
exec zsh
```

## 实现状态

| 模块 | 状态 | 说明 |
|------|------|------|
| ax 命令管理器 | ✅ 完成 | 添加/编辑/删除/执行/自动补全/自动同步 |
| WezTerm 配置 | ✅ 完成 | Catppuccin Mocha，tmux 风格快捷键 |
| zsh 配置 | ✅ 完成 | 自动建议、语法高亮、增强补全 |
| Starship Prompt | ✅ 完成 | 通过 install.sh 安装 |
| fzf 模糊搜索 | ✅ 完成 | 通过 install.sh 安装 |
| 一键部署脚本 | ✅ 完成 | install.sh |
| 命令库远程同步 | ✅ 完成 | add/edit/rm 后自动 commit + push |
| Git 配置 | 🔲 待配置 | 需要填充 .gitconfig |
| tmux 配置 | 🔲 待实现 | 预留，后续需要时添加 |
| Neovim / IDE 配置 | 🔲 待规划 | 按需添加 |
| 自定义脚本 | 🔲 待规划 | 按需添加 |

## 仓库结构

```
ax-system-basic/
├── install.sh              # 一键部署
├── apt-packages.txt        # 系统包列表
├── ax-commands.json        # 自定义命令库（自动同步）
├── bin/
│   └── ax                  # 命令管理器
├── bash/
│   ├── .zshrc              # zsh 配置
│   ├── .bashrc             # bash 配置（兼容）
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
