#!/bin/bash
# install.sh - 一键部署开发环境
# 支持: Ubuntu/Debian, Fedora/RHEL, Arch/Manjaro
# 优先使用预编译 Rust 二进制，回退到 bash 版
set -e

DOTDIR="$HOME/.ax"
BACKUP_DIR="$HOME/.ax-backup-$(date +%Y%m%d%H%M%S)"

# === 加载系统检测 ===
source "$DOTDIR/lib/detect.sh"
source "$DOTDIR/lib/packages.sh"
source "$DOTDIR/lib/shell.sh"
source "$DOTDIR/lib/tools.sh"
source "$DOTDIR/lib/deploy.sh"

echo "🚀 开始部署开发环境..."
log_os

# === 1. 安装 ax 工具 ===
echo ""
echo "🔧 安装 ax CLI..."

AX_BIN="$DOTDIR/bin/ax-rs/target/release/ax"

# 如果有预编译二进制
if [ -f "$AX_BIN" ]; then
  echo "  ✅ 使用本地预编译二进制"
else
  # 尝试从 release 下载
  OS=$(uname -s | tr '[:upper:]' '[:lower:]')
  ARCH=$(uname -m)

  case "$ARCH" in
    x86_64|amd64) ARCH_SUFFIX="x86_64" ;;
    aarch64|arm64) ARCH_SUFFIX="aarch64" ;;
    *) echo "  ⚠️  不支持的架构: $ARCH"; AX_BIN="" ;;
  esac

  if [ -n "$ARCH_SUFFIX" ]; then
    case "$OS" in
      linux) ARTIFACT="ax-linux-${ARCH_SUFFIX}" ;;
      darwin) ARTIFACT="ax-macos-${ARCH_SUFFIX}" ;;
      *) ARTIFACT="" ;;
    esac
  fi

  if [ -n "$ARTIFACT" ]; then
    echo "  📥 下载预编译二进制: ${ARTIFACT}"
    TMP_FILE=$(mktemp)
    if curl -fLo "$TMP_FILE" "https://anyhub.yushe.ai/leiax00/ax-system-basic/releases/latest/download/${ARTIFACT}" 2>/dev/null; then
      mkdir -p "$DOTDIR/bin"
      cp "$TMP_FILE" "$DOTDIR/bin/ax"
      chmod +x "$DOTDIR/bin/ax"
      rm "$TMP_FILE"
      AX_BIN="$DOTDIR/bin/ax"
      echo "  ✅ 下载完成"
    else
      echo "  ⚠️  下载失败，使用 bash 版"
      AX_BIN="$DOTDIR/bin/ax"
    fi
  fi
fi

# 部署 ax 到 ~/.local/bin
mkdir -p "$HOME/.local/bin"
if [ -f "$DOTDIR/bin/ax-rs/target/release/ax" ]; then
  ln -sf "$DOTDIR/bin/ax-rs/target/release/ax" "$HOME/.local/bin/ax"
elif [ -f "$DOTDIR/bin/ax" ]; then
  chmod +x "$DOTDIR/bin/ax"
  ln -sf "$DOTDIR/bin/ax" "$HOME/.local/bin/ax"
fi
echo "  ✅ ax → ~/.local/bin/ax"

# === 2. 安装系统包 ===
install_packages

# === 3. 安装 zsh + 插件 ===
install_zsh
set_default_shell
install_zsh_plugins

# === 4. 安装工具 ===
install_fzf
install_starship
install_nerd_font

# === 5. 部署配置文件 ===
deploy_dotfiles
deploy_ax_tool

echo ""
echo "✅ 部署完成！"
echo "📁 原有配置已备份到: $BACKUP_DIR"
echo ""
echo "👉 请重启终端，或运行: exec zsh"
