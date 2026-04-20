# 扩展指南

## 新增 CLI 命令

以添加 `ax hello` 命令为例：

### 步骤 1：定义命令 (cli.rs)

```rust
// 在 Commands 枚举中添加
pub enum Commands {
    // ... 已有命令
    Hello {
        #[arg(short, long)]
        name: Option<String>,
    },
}
```

### 步骤 2：实现逻辑 (commands/hello.rs)

```rust
use anyhow::Result;

pub fn execute(name: Option<&str>) -> Result<()> {
    match name {
        Some(n) => println!("Hello, {}!", n),
        None => println!("Hello, World!"),
    }
    Ok(())
}
```

### 步骤 3：注册路由 (main.rs)

```rust
// 在 mod 声明中添加
mod hello;

// 在 match 中添加
Commands::Hello { name } => hello::execute(name.as_deref())?,
```

## 新增 OS/发行版支持

### 步骤 1：添加 OS 映射 (detect.rs)

在 `os_id()` 函数的匹配中添加新的 OS ID：

```rust
"opensuse" => "opensuse",
```

### 步骤 2：添加包管理器映射

在 `pkg_manager()` 中添加：

```rust
"opensuse" => "zypper",
```

### 步骤 3：添加包列表

创建 `config/packages/opensuse.txt`：

```
# === core ===
git
curl
zsh
unzip
fontconfig

# === extras ===
jq
fzf
```

### 步骤 4：处理包检查

如果包管理器使用非标准查询方式，在 `is_package_installed()` 中添加对应逻辑。

## 新增配置模板

在 `config.rs` 中定义新的模板常量，并在 `ax config init` 流程中写入。

## 新增 Shell 插件

在 `config.yaml` 的 `shell.plugins` 中添加：

```yaml
plugins:
  - name: zsh-completions
    repo: https://github.com/zsh-users/zsh-completions
```
