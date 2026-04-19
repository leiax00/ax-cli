# 部署与更新

## 快速部署

```bash
# 下载二进制
curl -fLo ~/.local/bin/ax https://anyhub.yushe.ai/.../ax-linux-x86_64
chmod +x ~/.local/bin/ax

# 初始化 + 安装
ax config init
ax install
exec zsh
```

不需要 clone 源码仓库，配置模板全部内置在二进制里。

## 支持的发行版

自动检测系统，无需手动选择：

| 发行版 | 包管理器 | 包列表 |
|--------|---------|--------|
| Ubuntu / Debian / Linux Mint | apt | `packages/ubuntu.txt` |
| Fedora / RHEL / CentOS | dnf | `packages/fedora.txt` |
| Arch / Manjaro | pacman | `packages/arch.txt` |

## 编译平台

| 平台 | 目标 | 二进制名 |
|------|------|---------|
| Linux x86_64 | x86_64-unknown-linux-gnu | `ax-linux-x86_64` |
| Linux aarch64 | aarch64-unknown-linux-gnu | `ax-linux-aarch64` |
| macOS Intel | x86_64-apple-darwin | `ax-macos-x86_64` |
| macOS Apple Silicon | aarch64-apple-darwin | `ax-macos-aarch64` |
| Windows | x86_64-pc-windows-msvc | `ax-windows-x86_64.exe` |

### 本地编译

```bash
cd bin/ax-rs
cargo build --release
# 产物: target/release/ax (~4.6MB)
```

### 发布

打 tag 即触发 CI/CD（GitHub Actions + Gitea Actions 兼容）：

```bash
git tag v0.1.0
git push origin v0.1.0
```

## ax install 执行流程

| 步骤 | 内容 |
|------|------|
| 1 | 安装系统包（自动检测发行版） |
| 2 | 安装 zsh + 切换默认 shell + 安装插件 |
| 3 | 安装 fzf / Starship / 字体 |
| 4 | 部署配置文件（符号链接） |

## 备份

首次部署时，已有配置会备份到 `~/.ax-backup-<时间戳>/`。

## 日常更新

```bash
ax pull        # 拉取配置 + 检查包 + 更新插件
exec zsh       # 生效
```

## 新增系统包

编辑包列表文件：

```bash
echo "新包名" >> ~/.config/ax-cli/packages/ubuntu.txt
ax config push
```

其他机器运行 `ax pull` 即可安装。

---

**返回** → [模块列表](./README.md)
