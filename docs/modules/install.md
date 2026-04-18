# 部署与更新

## 一键部署

```bash
git clone https://anyhub.yushe.ai/leiax00/ax-system-basic.git ~/.dotfiles
~/.dotfiles/install.sh
exec zsh
```

## 支持的发行版

自动检测系统，无需手动选择：

| 发行版 | 包管理器 | 包列表 |
|--------|---------|--------|
| Ubuntu / Debian / Linux Mint | apt | `packages/ubuntu.txt` |
| Fedora / RHEL / CentOS | dnf | `packages/fedora.txt` |
| Arch / Manjaro | pacman | `packages/arch.txt` |

## install.sh 执行流程

通过 `lib/` 模块按顺序执行：

| 步骤 | 模块 | 内容 |
|------|------|------|
| 1 | `lib/packages.sh` | 安装系统包（自动检测发行版） |
| 2 | `lib/shell.sh` | 安装 zsh |
| 3 | `lib/shell.sh` | 切换默认 shell |
| 4 | `lib/shell.sh` | 安装 zsh 插件 |
| 5 | `lib/tools.sh` | 安装 fzf |
| 6 | `lib/tools.sh` | 安装 Starship |
| 7 | `lib/tools.sh` | 安装 JetBrains Mono Nerd Font |
| 8 | `lib/deploy.sh` | 链接配置文件 |
| 9 | `lib/deploy.sh` | 部署 ax 工具 + 命令库 |

## 备份

首次部署时，已有配置会备份到 `~/.dotfiles-backup-<时间戳>/`。

## ax update

```bash
ax update
```

自动执行：
1. 拉取 dotfiles 仓库最新代码
2. 刷新 ax 工具链接
3. 检查并安装新增系统包
4. 更新 zsh 插件
5. 检查字体

更新后运行 `exec zsh` 使配置生效。

## 手动更新

```bash
cd ~/.dotfiles && git pull
exec zsh
```

## 新增系统包

编辑对应发行版的包列表文件并提交：

```bash
# Ubuntu
echo "新包名" >> ~/.dotfiles/packages/ubuntu.txt

# Fedora
echo "新包名" >> ~/.dotfiles/packages/fedora.txt

# 提交
cd ~/.dotfiles && git add packages/ && git commit -m "add 新包名" && git push
```

其他机器运行 `ax update` 即可安装。

---

**返回** → [模块列表](./README.md)
