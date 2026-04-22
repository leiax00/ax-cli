# 配置系统设计

## 概述

配置系统是 ax-cli 的核心基础设施，负责配置的加载、合并、模板生成和持久化存储。

## 目录结构

```
~/.config/ax-cli/          # 默认配置根目录
├── config.yaml            # 主配置文件（部署链接、包列表路径等）
├── config.d/              # 动态数据目录
│   ├── commands.yaml      # 用户自定义命令
│   ├── env.yaml           # 用户环境变量
│   └── ssh.yaml           # 用户 SSH 主机配置
└── .git/                  # 配置仓库（用于同步）
```

## 配置优先级

从高到低：

1. **`AX_CONFIG_DIR` 环境变量**：用户显式指定的配置目录
2. **便携模式**：可执行文件同级的 `config/` 目录
3. **默认路径**：`~/.config/ax-cli/`

实现位置：`ConfigLoader::load()` (`src/config.rs`)

## 配置合并策略

### 主配置层级

`config.yaml` 定义全局配置，`config.d/` 下的文件作为补充：

- `config.yaml`：静态配置（部署目标、包列表路径、shell 插件等）
- `config.d/commands.yaml`：用户动态命令数据
- `config.d/env.yaml`：用户动态环境变量数据
- `config.d/ssh.yaml`：用户动态 SSH 主机数据

### 合并规则

- 命令加载：`load_all_commands()` 合并主配置中的 `commands` 字段和 `config.d/commands.yaml`
- 环境变量：独立存储在 `config.d/env.yaml`，不与主配置合并
- SSH 主机：独立存储在 `config.d/ssh.yaml`

## 主配置结构 (Config)

```yaml
# config.yaml 示例
shell:
  default: zsh
  plugins:
    - name: zsh-autosuggestions
      repo: https://github.com/zsh-users/zsh-autosuggestions
    - name: zsh-syntax-highlighting
      repo: https://github.com/zsh-users/zsh-syntax-highlighting

packages:
  path: packages/{os}.txt

deploy:
  - source: bash/.bashrc
    target: ~/.bashrc
  - source: bash/.zshrc
    target: ~/.zshrc
  - source: wezterm/wezterm.lua
    target: ~/.config/wezterm/wezterm.lua

commands:
  - name: example
    cmd: echo "hello"
    desc: "示例命令"

proxy:
  host: "127.0.0.1"
  port: "7890"
```

## 命令存储结构 (CommandEntry)

```yaml
# config.d/commands.yaml
- name: deploy
  cmd: kubectl apply -k overlays/production
  desc: 部署到生产环境
```

## 环境变量存储结构 (EnvEntry)

```yaml
# config.d/env.yaml
- name: AWS_ACCESS_KEY_ID
  value: "xxx"
  tags: [aws, production]
  paused: false
```

## SSH 主机存储结构 (SshHostEntry)

```yaml
# config.d/ssh.yaml
lax-tsj:
  host: lax-tsj
  user: user
  port: 22
  auth: password
  password: "xxx"
  key: ""
  desc: 洛杉矶云主机
```

## 内置模板

`config.rs` 中定义了以下配置模板（`ax config init` 时写入）：

| 模板常量 | 生成文件 | 内容 | 来源 |
|---------|---------|------|------|
| `TEMPLATE_CONFIG_YAML` | `config.yaml` | 主配置示例 | 代码内嵌 |
| `TEMPLATE_ZSHRC` | `~/.config/axconfig/bash/.zshrc` | zsh 托管配置片段（由 `~/.zshrc` 引入） | `config/bash/.zshrc` |
| `TEMPLATE_BASHRC` | `~/.config/axconfig/bash/.bashrc` | bash 托管配置片段（由 `~/.bashrc` 引入） | `config/bash/.bashrc` |
| `TEMPLATE_WEZTERM` | 部署为 `~/.config/wezterm/wezterm.lua` | WezTerm 终端配置 | `config/wezterm/wezterm.lua` |
| `TEMPLATE_TMUX` | 部署为 `~/.config/tmux/tmux.conf` | tmux 终端复用配置 | `config/tmux/tmux.conf` |

## 配置同步机制

- 基于 git：`config init` 时自动 `git init`
- `push`：`git add -A && git commit && git push`
- `pull`：`git pull`
- 配置修改只写入本地目录；需要同步到远程时手动执行 `ax push` 或 `ax config push`

## 导入导出

- `export`：将配置目录打包为 `tar.gz`
- `import`：解压 `tar.gz` 到配置目录（覆盖已有文件）
