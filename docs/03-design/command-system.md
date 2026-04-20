# 命令系统设计

## 概述

ax-cli 使用 clap derive 模式定义 CLI 结构，通过枚举路由到各命令实现模块。运行时会基于语言环境生成本地化后的命令树，该命令树同时用于：

- 命令行参数解析
- `--help` 文案展示
- shell 补全脚本生成

## CLI 结构定义

定义位置：`src/cli.rs`

```
Cli (顶层)
├── Commands (主命令)
│   ├── Install
│   ├── Config(ConfigAction)
│   │   ├── Init { force }
│   │   ├── Remote { url }
│   │   ├── Push
│   │   ├── Pull
│   │   ├── Export { output }
│   │   ├── Import { file }
│   │   └── Path
│   ├── Add { name, cmd, desc }
│   ├── Edit { name }
│   ├── Rm { name }
│   ├── List
│   ├── Run { name? }
│   ├── Env(EnvAction)
│   │   ├── Add { name, value, tag? }
│   │   ├── Edit { name }
│   │   ├── Rm { name }
│   │   ├── Show { name? | tag? | all }
│   │   ├── Tags
│   │   ├── Pause { name | tag }
│   │   ├── Resume { name | tag }
│   │   └── Load
│   ├── Proxy(ProxyAction)
│   │   ├── On
│   │   ├── Off
│   │   └── Status
│   ├── Push (别名)
│   ├── Pull (别名)
│   ├── Update
│   ├── Info
│   └── Completion { shell }
```

## 路由机制

`src/main.rs` 中的 `main()` 函数通过 `match` 将 `Commands` 枚举分发到对应的处理函数：

- `Commands::Install` → `commands::install::execute()`
- `Commands::Config(action)` → `commands::config::execute(action)`
- `Commands::Add/Edit/Rm/List/Run` → `commands::{add,edit,rm,list,run}::execute()`
- `Commands::Env(action)` → `commands::env::execute(action)`
- `Commands::Proxy(action)` → `commands::proxy::execute(action)`
- 其他 → 对应模块函数

## 本地化命令树

`src/cli.rs` 除了静态 derive 定义，还提供：

- `current_language()`：从 `AX_LANG`、`LC_ALL`、`LC_MESSAGES`、`LANG` 推断语言
- `localized_command()`：生成带本地化说明的 `clap::Command`
- `parse()`：使用本地化命令树进行参数解析

当前支持中文和英文两套命令说明。

## 新增命令的三步模式

1. **`src/cli.rs`**：在 `Commands` 枚举中添加新变体
2. **`src/commands/<name>.rs`**：实现 `execute()` 函数
3. **`src/main.rs`**：在 `match` 中添加路由分支

## 命令实现规范

- 每个命令文件独立，函数签名统一为 `pub fn execute(...) -> Result<()>`
- 错误处理统一使用 `anyhow::Result`
- 路径展开使用 `expand()` 处理 `~` 前缀
- 修改配置后通过 `save_commands()` / `save_env()` 持久化
