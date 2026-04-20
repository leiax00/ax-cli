#!/usr/bin/env bash
# ax-cli 一键安装脚本
# 自动检测平台，下载对应二进制到 ~/.local/bin
# 用法: curl -fsSL https://anyhub.yushe.ai/leiax00/ax-system-basic/raw/branch/main/install/install-ax.sh | bash

set -euo pipefail

# ========== 配置 ==========
REPO="leiax00/ax-system-basic"
GITEA_BASE="https://anyhub.yushe.ai"
INSTALL_DIR="$HOME/.local/bin"
BINARY_NAME="ax"

# 颜色
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

info()  { echo -e "${GREEN}[INFO]${NC} $*"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
error() { echo -e "${RED}[ERROR]${NC} $*" >&2; exit 1; }

# ========== 平台检测 ==========
detect_platform() {
    local os arch

    case "$(uname -s)" in
        Linux*)   os="linux" ;;
        Darwin*)  os="macos" ;;
        MINGW*|MSYS*|CYGWIN*) os="windows" ;;
        *) error "不支持的操作系统: $(uname -s)" ;;
    esac

    case "$(uname -m)" in
        x86_64|amd64)  arch="x86_64" ;;
        aarch64|arm64) arch="aarch64" ;;
        *) error "不支持的架构: $(uname -m)" ;;
    done

    echo "${os}-${arch}"
}

# ========== 下载 ==========
download_ax() {
    local platform="$1"
    local version="${2:-latest}"
    local url filename

    if [ "$os" = "windows" ]; then
        filename="${BINARY_NAME}.exe"
    else
        filename="${BINARY_NAME}"
    fi

    if [ "$version" = "latest" ]; then
        # 从 Gitea API 获取最新 release
        info "查询最新版本..."
        local api_url="${GITEA_BASE}/api/v1/repos/${REPO}/releases"
        local release_info
        release_info=$(curl -fsSL "$api_url" 2>/dev/null | head -1) || true

        if [ -z "$release_info" ]; then
            # API 不可用，尝试直接下载 main 分支编译产物
            warn "无法获取 release 信息，尝试下载预编译版本..."
            url="${GITEA_BASE}/${REPO}/releases/download/latest/ax-${platform}${ext}"
        else
            url="${GITEA_BASE}/${REPO}/releases/latest/download/ax-${platform}${ext}"
        fi
    else
        url="${GITEA_BASE}/${REPO}/releases/download/${version}/ax-${platform}${ext}"
    fi

    local tmp_file
    tmp_file="$(mktemp)"

    info "下载: $url"
    if ! curl -fSL --progress-bar -o "$tmp_file" "$url" 2>&1; then
        # 尝试备用下载地址（GitHub）
        local gh_url="https://github.com/${REPO}/releases/${version}/download/ax-${platform}${ext}"
        info "备用下载: $gh_url"
        if ! curl -fSL --progress-bar -o "$tmp_file" "$gh_url" 2>&1; then
            rm -f "$tmp_file"
            error "下载失败，请手动下载: ${GITEA_BASE}/${REPO}/releases"
        fi
    fi

    chmod +x "$tmp_file"
    echo "$tmp_file"
}

# ========== 安装 ==========
install_ax() {
    local platform="$1"
    local tmp_file="$2"
    local dest="${INSTALL_DIR}/${BINARY_NAME}"

    # Windows 特殊处理
    if [ "$os" = "windows" ]; then
        dest="${INSTALL_DIR}/${BINARY_NAME}.exe"
    fi

    # 创建安装目录
    mkdir -p "$INSTALL_DIR"

    # 备份已有版本
    if [ -f "$dest" ]; then
        local backup="${dest}.bak.$(date +%Y%m%d%H%M%S)"
        mv "$dest" "$backup"
        info "已备份旧版本: $backup"
    fi

    # 安装
    mv "$tmp_file" "$dest"
    info "已安装: $dest"
}

# ========== PATH 检查 ==========
check_path() {
    case ":$PATH:" in
        *":${INSTALL_DIR}:"*) ;;
        *)
            warn "安装目录不在 PATH 中!"
            echo ""
            echo "  请将以下内容添加到你的 shell 配置文件:"
            echo ""
            if command -v ax &>/dev/null && ax env load 2>/dev/null | grep -q '\$env:'; then
                # PowerShell
                echo '  $env:PATH += ";'"$INSTALL_DIR"'"'
            else
                echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
            fi
            echo ""
            ;;
    esac
}

# ========== 主流程 ==========
main() {
    local version="${1:-latest}"

    echo ""
    echo "  ╔══════════════════════════════╗"
    echo "  ║      ax-cli 安装程序          ║"
    echo "  ╚══════════════════════════════╝"
    echo ""

    local platform
    platform=$(detect_platform)
    info "平台: $platform"

    local tmp_file
    tmp_file=$(download_ax "$platform" "$version")

    install_ax "$platform" "$tmp_file"

    check_path

    # 验证
    echo ""
    if command -v "$BINARY_NAME" &>/dev/null; then
        local installed_version
        installed_version=$("$BINARY_NAME" --version 2>/dev/null || echo "unknown")
        info "安装成功! ax ${installed_version}"
    else
        info "安装完成! 请重启终端或手动 source 配置"
    fi

    echo ""
    echo "  下一步:"
    echo "    ax config init    # 初始化配置"
    echo "    ax install        # 安装系统包和工具"
    echo ""
}

main "$@"
