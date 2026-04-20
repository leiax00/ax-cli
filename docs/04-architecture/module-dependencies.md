# 模块依赖关系

## 模块依赖图

```
main.rs
  ├── cli.rs
  ├── commands/install.rs
  │     ├── packages.rs
  │     │     ├── detect.rs
  │     │     └── config.rs
  │     ├── shell.rs
  │     │     ├── config.rs
  │     │     └── which (crate)
  │     ├── tools.rs
  │     │     └── which (crate)
  │     └── config.rs
  ├── commands/config.rs
  │     └── config.rs
  ├── commands/env.rs
  │     └── config.rs
  ├── commands/proxy.rs
  │     └── config.rs
  ├── commands/add.rs, edit.rs, rm.rs, list.rs, run.rs
  │     └── config.rs
  ├── commands/info.rs
  │     ├── config.rs
  │     └── detect.rs
  └── commands/completion.rs
        └── cli.rs
```

## 外部 crate 依赖

| Crate | 用途 | 使用模块 |
|-------|------|---------|
| clap | CLI 参数解析 | cli.rs, main.rs |
| serde + serde_yaml | YAML 序列化 | config.rs |
| anyhow | 错误处理 | 全部模块 |
| dirs | 系统目录检测 | config.rs |
| which | 可执行文件检测 | detect.rs, shell.rs, tools.rs |
| git2 | Git 操作 | commands/config.rs |
| regex | 正则匹配 | commands/ 中多处 |
| log + env_logger | 日志 | 全局 |

## 接口约定

### config.rs 对外接口

```rust
fn load() -> Result<Config>                              // 加载主配置
fn load_all_commands() -> Result<Vec<CommandEntry>>       // 加载所有命令
fn save_commands(cmds: &[CommandEntry]) -> Result<()>     // 保存命令
fn load_env() -> Result<Vec<EnvEntry>>                    // 加载环境变量
fn save_env(envs: &[EnvEntry]) -> Result<()>              // 保存环境变量
fn config_dir() -> Result<PathBuf>                        // 获取配置目录
```

### detect.rs 对外接口

```rust
fn os_id() -> &'static str          // OS 标识符
fn os_name() -> String              // OS 全名
fn pkg_manager() -> &'static str    // 包管理器名称
fn packages_file() -> String        // 包列表文件名
fn is_package_installed(pkg: &str) -> bool  // 包是否已安装
```

### shell.rs 对外接口

```rust
fn install_zsh() -> Result<()>           // 安装 zsh
fn set_default_shell() -> Result<()>     // 设置默认 shell
fn install_plugins(plugins: &[ShellPlugin]) -> Result<()>  // 安装插件
fn update_plugins(plugins: &[ShellPlugin]) -> Result<()>   // 更新插件
```

### tools.rs 对外接口

```rust
fn install_fzf() -> Result<()>     // 安装 fzf
fn install_starship() -> Result<()> // 安装 starship
fn check_font() -> Result<()>       // 检查/安装 Nerd Font
```

### packages.rs 对外接口

```rust
fn check_and_install(extras: bool) -> Result<()>  // 检查并安装系统包
```
