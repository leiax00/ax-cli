# dotfiles

个人开发环境配置，Ubuntu 24.04 LTS。

## 一键部署

```bash
git clone git@github.com:你的用户名/dotfiles.git ~/.dotfiles
~/.dotfiles/install.sh
source ~/.bashrc
```

## 包含内容

- **ax** - 自定义命令管理器，支持添加/编辑/删除/搜索/自动补全
- **WezTerm** - 终端配置（Catppuccin Mocha 主题，tmux 风格快捷键）
- **bash** - shell 配置 + 补全
- **Starship** - 漂亮的 prompt
- **fzf** - 模糊搜索
- **apt 包列表** - 一键安装常用开发工具

## 快捷键 (WezTerm)

Leader: `Ctrl+A`

| 快捷键 | 功能 |
|--------|------|
| `Ctrl+A c` | 新标签页 |
| `Ctrl+A n/p` | 切换标签 |
| `Ctrl+A \|` | 水平分屏 |
| `Ctrl+A -` | 垂直分屏 |
| `Ctrl+A h/j/k/l` | 切换面板 |
| `Ctrl+A z` | 全屏切换 |
| `Ctrl+A r` | 重载配置 |

## ax 命令管理

```bash
ax add <名称> <命令> [描述]    # 添加
ax edit <名称>                  # 编辑
ax list                        # 列表
ax rm <名称>                   # 删除
ax <名称>                      # 执行
ax                             # 交互选择
```
