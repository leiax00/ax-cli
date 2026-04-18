#!/bin/bash
# lib/detect.sh - 系统检测
# 用法: source lib/detect.sh

_detect_os() {
  if [ -f /etc/os-release ]; then
    . /etc/os-release
    OS_ID="$ID"
    OS_NAME="$NAME"
  elif command -v lsb_release &>/dev/null; then
    OS_ID=$(lsb_release -si | tr '[:upper:]' '[:lower:]')
    OS_NAME=$(lsb_release -sd)
  else
    OS_ID="unknown"
    OS_NAME="Unknown"
  fi
}

_detect_pkg_manager() {
  case "$OS_ID" in
    ubuntu|debian|linuxmint|pop)
      PKG_MANAGER="apt"
      PKG_CHECK="dpkg -s"
      PKG_UPDATE="sudo apt update -qq"
      PKG_INSTALL="sudo apt install -y -qq"
      PKG_LIST_FILE="packages/apt.txt"
      ;;
    fedora|rhel|centos|rocky|alma)
      PKG_MANAGER="dnf"
      PKG_CHECK="rpm -q"
      PKG_UPDATE="sudo dnf check-update --quiet"
      PKG_INSTALL="sudo dnf install -y"
      PKG_LIST_FILE="packages/dnf.txt"
      ;;
    arch|manjaro|endeavouros)
      PKG_MANAGER="pacman"
      PKG_CHECK="pacman -Q"
      PKG_UPDATE="sudo pacman -Sy --noconfirm"
      PKG_INSTALL="sudo pacman -S --noconfirm --needed"
      PKG_LIST_FILE="packages/pacman.txt"
      ;;
    *)
      PKG_MANAGER="unknown"
      PKG_LIST_FILE=""
      ;;
  esac
}

# 执行检测
_detect_os
_detect_pkg_manager

log_os() {
  echo "🖥️  系统: $OS_NAME ($OS_ID)"
  echo "📦 包管理器: $PKG_MANAGER"
}
