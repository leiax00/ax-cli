# 部署与更新

## 一键部署

```bash
git clone https://anyhub.yushe.ai/leiax00/ax-system-basic.git ~/.ax
~/.ax/install.sh
exec zsh
```

## 支持的发行版

自动检测系统，无需手动选择：

| 发行版 | 包管理器 | 包列表 |
|--------|---------|--------|
| Ubuntu / Debian / Linux Mint | apt | `packages/ubuntu.txt` |
| Fedora / RHEL / CentOS | dnf | `packages/fedora.txt` |
| Arch / Manjaro | pacman | `packages/arch.txt` |

## ax CLI（Rust 版）

`ax` 现在是 Rust 编译的单二进制工具，零运行时依赖。

### 构建平台

| 平台 | 目标 | 二进制名 |
|------|------|---------|
| Linux x86_64 | x86_64-unknown-linux-gnu | `ax-linux-x86_64` |
| Linux aarch64 | aarch64-unknown-linux-gnu | `ax-linux-aarch64` |
| macOS Intel | x86_64-apple-darwin | `ax-macos-x86_64` |
| macOS Apple Silicon | aarch64-apple-darwin | `ax-macos-aarch64` |
| Windows | x86_64-pc-windows-msvc | `ax-windows-x86_64.exe` |

### 本地编译

```bash
cd ~/.ax/bin/ax-rs
cargo build --release
# 产物: target/release/ax
```

### 发布流程

打 tag 即触发 CI/CD 自动编译和发布：

```bash
cd ~/.ax
git tag v0.1.0
git push origin v0.1.0
```

CI 使用 GitHub Actions / Gitea Actions（兼容）。

## install.sh 执行流程

| 步骤 | 内容 |
|------|------|
| 1 | 安装 ax CLI（优先下载预编译二进制，回退到 bash 版） |
| 2 | 安装系统包（自动检测发行版） |
| 3 | 安装 zsh + 切换默认 shell + 安装插件 |
| 4 | 安装 fzf / Starship / 字体 |
| 5 | 链接配置文件 + 部署 ax 工具 |

## 备份

首次部署时，已有配置会备份到 `~/.ax-backup-<时间戳>/`。

## ax update

```bash
ax update
```

自动执行：
1. 拉取 dotfiles 仓库最新代码
2. 刷新 ax 工具链接
3. 检查并安装新增系统包
4. 更新 zsh 插件
5. 检查字体

## 新增系统包

编辑对应发行版的包列表文件并提交：

```bash
echo "新包名" >> ~/.ax/packages/ubuntu.txt
cd ~/.ax && git add packages/ && git commit -m "add 新包名" && git push
```

其他机器运行 `ax update` 即可安装。

---

**返回** → [模块列表](./README.md)
