# 环境变量管理设计

## 概述

`ax env` 提供带标签分组的环境变量管理，支持暂停/恢复和多 shell 输出。

## 数据模型

```rust
struct EnvEntry {
    name: String,       // 变量名
    value: String,      // 变量值
    tags: Vec<String>,  // 标签列表
    paused: bool,       // 是否暂停
}
```

存储位置：`config.d/env.yaml`

## 命令详解

### add

```bash
ax env add NAME VALUE [--tag tag1,tag2]
```

- 创建新的环境变量条目
- 标签可选，默认为空
- 保存后仅写入 `config.d/env.yaml`，如需同步请手动执行 `ax push`

### show

```bash
ax env show              # 显示所有未暂停的变量
ax env show --name FOO   # 按名称过滤
ax env show --tag aws    # 按标签过滤
ax env show --all        # 显示所有（含已暂停）
```

### pause / resume

```bash
ax env pause --name FOO      # 暂停指定变量
ax env pause --tag aws       # 暂停标签下所有变量
ax env resume --name FOO     # 恢复指定变量
ax env resume --tag aws      # 恢复标签下所有变量
```

### load

```bash
ax env load    # 输出 export 语句，用于 eval $(ax env load)
```

- 只输出未暂停的变量
- 自动检测当前 shell 类型
- bash/zsh：`export NAME=value`
- PowerShell：`$env:NAME="value"`
- CMD：`set NAME=value`
