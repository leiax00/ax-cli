# 配置系统设计

## 概述

配置系统是 ax-cli 的核心基础设施，负责配置的加载、合并、模板生成和持久化存储。

## 目录结构

```
~/.config/ax-cli/          # 默认配置根目录
├── config.yaml            # 主配置文件（部署链接、包列表路径等）
├── config.d/              # 动态数据目录
│   ├── commands.yaml      # 用户自定义命令
│   └── env.yaml           # 用户环境变量
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

### 合并规则

- 命令加载：`load_all_commands()` 合并主配置中的 `commands` 字段和 `config.d/commands.yaml`
- 环境变量：独立存储在 `config.d/env.yaml`，不与主配置合并

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

## 内置模板

`config.rs` 中定义了以下配置模板（`ax config init` 时写入）：

| 模板常量 | 生成文件 | 内容 |
|---------|---------|------|
| `TEMPLATE_CONFIG_YAML` | `config.yaml` | 主配置示例 |
| `TEMPLATE_ZSHRC` | 部署为 `~/.zshrc` | zsh 配置（历史、补全、fzf、别名、代理） |
| `TEMPLATE_BASHRC` | 部署为 `~/.bashrc` | bash 配置（类似 zshrc） |
| `TEMPLATE_WEZTERM` | 部署为 `~/.config/wezterm/wezterm.lua` | WezTerm 终端配置 |

## 配置同步机制

- 基于 git：`config init` 时自动 `git init`
- `push`：`git add -A && git commit && git push`
- `pull`：`git pull`
- 环境变量和命令修改后自动触发 git 同步（如果配置了 remote）

## 导入导出

- `export`：将配置目录打包为 `tar.gz`
- `import`：解压 `tar.gz` 到配置目录（覆盖已有文件）
