# 包说明

`ax install` 默认安装 `core` 组。`ax install --extras` 会额外安装 `config/packages/*.txt` 中的 `extras` 组。

## Core

- `git`：用于配置仓库操作、zsh 插件克隆，以及 `fzf` 的安装流程。
- `curl`：用于下载内容，例如 `starship` 安装脚本和 Nerd Font 字体压缩包。
- `zsh`：当前安装流程默认管理和切换的 shell。
- `unzip`：用于解压下载的字体压缩包。
- `fontconfig`：提供 `fc-list` 和 `fc-cache`，用于字体检测和缓存刷新。

## Extras

- `jq`：用于在脚本里处理 JSON，适合接口调试和命令行数据过滤。
- `fzf`：模糊选择器，适合历史命令、文件列表和交互式选择。
- `wget`：额外的下载工具，在某些系统或脚本里比 `curl` 更顺手。
- `tree`：以树状结构显示目录内容。
- `htop`：交互式进程查看器。
- `ripgrep`：高速递归文本搜索工具。
- `fd-find` / `fd`：高速文件查找工具。
- `bat`：带语法高亮的文件查看工具。
- `tmux`：终端多路复用工具，适合分屏和长时间会话。
