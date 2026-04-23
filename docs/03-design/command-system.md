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
│   ├── Link
│   ├── Ssh { name?, action? }
│   │   ├── Add { name, host, user, port?, auth, password?, key?, desc? }
│   │   ├── SetupKey { name, host, user, port?, password?, key?, desc? }
│   │   ├── List
│   │   ├── Rm { name }
│   │   └── Connect { name }
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
- `Commands::Ssh { .. }` → `commands::ssh::{add,setup_key,list,rm,connect}()`
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
- SSH 主机配置通过 `save_ssh_hosts()` 持久化到 `config.d/ssh.yaml`

## SSH 命令

`ax ssh` 同时支持“管理保存的 SSH 主机”和“按别名直接连接”两种入口：

```bash
ax ssh add lax-tsj --host lax-tsj --user user --auth password --password 'xxx'
ax ssh setup-key lax-tsj --host lax-tsj --user user --password 'xxx'
ax ssh hk-prod
ax ssh list
ax ssh rm hk-prod
```

- `ax ssh <name>`：按别名直接连接
- `ax ssh`：未传别名时进入交互选择（有 fzf 用 fzf 模糊搜索，否则回退到编号选择）
- `ax ssh setup-key <name>`：分发本地公钥并自动保存为 `auth=key` 连接
- `auth=key`：调用系统 `ssh`
- `auth=password`：优先调用 `sshpass`; 若不可用，则打印密码并退化为普通 `ssh`

## Shell 函数自动生成

自定义命令支持直接在 shell 中以函数形式调用（如直接输入 `portal` 而非 `ax run portal`）。

### 实现机制

`config.rs` 中的 `generate_command_functions()` 读取所有自定义命令，生成 shell 函数文件到 `config.d/commands.sh`。每个命令生成同名函数，内联命令内容：

```bash
code_self() {
  cd /workspace/code/self
}

portal() {
  set -a; source deploy/portal/.env; set +a
  cd server/biz-sevice/portal-start
  mvn spring-boot:run -Dspring-boot.run.profiles=dev
}
```

这种方式确保 `cd` 等内置命令在当前 shell 上下文执行。

### 自动更新

`ax add`、`ax edit`、`ax rm` 执行后会自动调用 `generate_command_functions()` 更新函数文件。

### 手动刷新

- `ax link` — 重新生成 `commands.sh`，输出 source 命令提示
- `source ~/.config/axconfig/config.d/commands.sh` — 在当前 shell 中重新加载

### 加载方式

shell 模板（`.zshrc`/`.bashrc`）末尾包含：
```bash
[ -f "$HOME/.config/axconfig/config.d/commands.sh" ] && source "$HOME/.config/axconfig/config.d/commands.sh"
```
