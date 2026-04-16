# 部署脚本

## 一键部署

```bash
git clone https://anyhub.yushe.ai/leiax00/ax-system-basic.git ~/.dotfiles
~/.dotfiles/install.sh
exec zsh
```

## install.sh 做了什么

按顺序执行：

| 步骤 | 内容 | 说明 |
|------|------|------|
| 1 | 安装 apt 包 | 从 `apt-packages.txt` 读取列表 |
| 2 | 切换默认 shell | bash → zsh |
| 3 | 安装 zsh 插件 | autosuggestions / syntax-highlighting / completions |
| 4 | 安装 Nerd Font | JetBrains Mono（仅首次） |
| 5 | 安装 fzf | 模糊搜索（仅首次） |
| 6 | 安装 Starship | Prompt 主题（仅首次） |
| 7 | 链接配置文件 | .zshrc、wezterm.lua、.gitconfig |
| 8 | 部署 ax 工具 | bin/ax → ~/.local/bin/ax |
| 9 | 链接命令库 | ax-commands.json（符号链接） |

## 备份

首次部署时，已有配置会备份到 `~/.dotfiles-backup-<时间戳>/`。

## 更新

```bash
cd ~/.dotfiles && git pull
exec zsh
```

配置文件都是符号链接，pull 后自动生效（ax 工具需要 `source ~/.zshrc` 或重启终端）。

## 系统包列表

`apt-packages.txt` 中包含：

```
jq, fzf, git, curl, wget, tree, htop, ripgrep, fd-find, bat, tmux, zsh, python3-pip, nodejs, npm, docker.io, docker-compose-v2
```

需要新增系统包时，编辑 `apt-packages.txt` 并提交。

---

**返回** → [模块列表](./README.md)
