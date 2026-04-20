# 多平台支持设计

## OS 检测机制

实现位置：`src/detect.rs`

通过读取 `/etc/os-release` 获取 `ID` 字段，映射为内部 OS 标识符。

## OS 映射表

| /etc/os-release ID | 内部标识 | 包管理器 |
|---------------------|---------|---------|
| ubuntu, debian, linuxmint, pop | ubuntu | apt |
| fedora, rhel, centos, rocky, almalinux | fedora | dnf |
| arch, manjaro, endeavouros | arch | pacman |
| darwin (macOS) | macos | brew |
| (其他) | unknown | - |

## 包列表文件

```
config/packages/
├── ubuntu.txt
├── fedora.txt
├── arch.txt
└── macos.txt
```

命名规则：`packages/{内部标识}.txt`

## 新增发行版支持

1. 在 `detect.rs` 的 `os_id()` 中添加映射
2. 创建 `config/packages/{os_id}.txt`
3. 如包管理器不同于已有映射，在 `pkg_manager()` 中添加

## 包检查机制

- **apt/dnf/pacman**：通过 `which` 或包管理器 query 检查
- **brew**：`brew list` 检查

## 交叉编译

CI/CD 支持 5 个目标平台：

- `x86_64-unknown-linux-gnu`
- `aarch64-unknown-linux-gnu`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- `x86_64-pc-windows-msvc`
