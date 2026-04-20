# 补全与本地化设计

## 概述

`ax` 的命令补全和帮助说明共用同一棵 `clap` 命令树，避免出现“实际命令已经变更，但补全脚本仍然是旧规则”的问题。

当前目标：

- 支持多层子命令补全
- zsh 显示命令说明
- help 与补全说明支持中文 / 英文切换
- `ax install` 后补全与 shell 配置自动接入

## 补全生成策略

实现位置：`src/commands/completion.rs`

- 使用 `clap_complete` 在运行时生成 bash / zsh / PowerShell 补全脚本
- 补全脚本来源于 `src/cli.rs` 的真实命令树
- 新增或修改子命令后，不需要再手写补全规则

支持的安装路径：

- bash: `~/.local/share/bash-completion/completions/ax`
- zsh: `~/.zsh/completions/_ax` 或 `~/.local/share/zsh/site-functions/_ax`
- PowerShell: `~/Documents/PowerShell/Microsoft.PowerShell_profile.ps1`

## shell 接入

托管 shell 片段定义在 `src/config.rs`：

- `TEMPLATE_ZSHRC`
- `TEMPLATE_BASHRC`

其中包含：

- `ax` shell function
- bash / zsh 补全加载
- zsh `fpath` 配置
- `zsh-autosuggestions` / `zsh-syntax-highlighting` / `zsh-completions` 插件加载

`ax install` 在 `src/commands/install.rs` 中会执行：

1. 刷新托管的 `bash/.zshrc` 和 `bash/.bashrc`
2. 安装 zsh / bash 补全脚本
3. 确保用户的 `~/.zshrc` / `~/.bashrc` 引入托管片段

## 语言选择

实现位置：`src/cli.rs`

语言选择顺序：

1. `AX_LANG`
2. `LC_ALL`
3. `LC_MESSAGES`
4. `LANG`
5. 默认 `zh`

当前支持：

- `zh` / `zh_CN` → 中文
- `en` / `en_US` → 英文

## 本地化方式

`src/cli.rs` 提供：

- `current_language()`：读取环境变量，决定当前语言
- `localized_command()`：生成带本地化说明的 `clap::Command`
- `parse()`：使用本地化后的命令树进行参数解析

这样有两个收益：

- `ax --help` 与 `ax env --help` 直接显示对应语言
- `ax completion zsh` 生成的说明文本和 `--help` 保持一致

## 已知边界

- 静态命令和参数说明已经多语言化
- 运行时 `println!` 输出仍以中文为主，尚未统一接入语言表
- 动态值补全（例如用户自定义命令名、环境变量名）后续可叠加在 `clap_complete` 基础上扩展
