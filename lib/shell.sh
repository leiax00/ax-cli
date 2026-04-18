#!/bin/bash
# lib/shell.sh - zsh 配置
# 依赖: 无

ZSH_PLUGIN_DIR="$HOME/.zsh/plugins"

ZSH_PLUGINS=(
  "zsh-autosuggestions|https://github.com/zsh-users/zsh-autosuggestions"
  "zsh-syntax-highlighting|https://github.com/zsh-users/zsh-syntax-highlighting"
  "zsh-completions|https://github.com/zsh-users/zsh-completions"
)

install_zsh() {
  echo ""
  echo "🐚 安装 zsh..."
  if command -v zsh &>/dev/null; then
    echo "  ⏭️  zsh 已安装"
  else
    if [ "$PKG_MANAGER" = "apt" ]; then
      sudo apt install -y -qq zsh
    elif [ "$PKG_MANAGER" = "dnf" ]; then
      sudo dnf install -y zsh
    elif [ "$PKG_MANAGER" = "pacman" ]; then
      sudo pacman -S --noconfirm zsh
    fi
    echo "  ✅ zsh 安装完成"
  fi
}

set_default_shell() {
  echo ""
  echo "🐚 设置 zsh 为默认 shell..."
  if [ "$SHELL" != "$(which zsh)" ]; then
    chsh -s "$(which zsh)"
    echo "  ✅ 默认 shell 已切换为 zsh（重启终端生效）"
  else
    echo "  ⏭️  zsh 已是默认 shell"
  fi
}

install_zsh_plugins() {
  echo ""
  echo "🔌 安装 zsh 插件..."
  mkdir -p "$ZSH_PLUGIN_DIR"

  for plugin in "${ZSH_PLUGINS[@]}"; do
    local name="${plugin%%|*}"
    local url="${plugin##*|}"
    if [ -d "$ZSH_PLUGIN_DIR/$name" ]; then
      echo "  ⏭️  $name 已存在"
    else
      git clone --depth 1 "$url" "$ZSH_PLUGIN_DIR/$name" 2>&1 | tail -1
      echo "  ✅ $name"
    fi
  done
}

update_zsh_plugins() {
  echo ""
  echo "🔌 更新 zsh 插件..."
  mkdir -p "$ZSH_PLUGIN_DIR"

  for plugin in "${ZSH_PLUGINS[@]}"; do
    local name="${plugin%%|*}"
    local url="${plugin##*|}"
    if [ -d "$ZSH_PLUGIN_DIR/$name" ]; then
      (cd "$ZSH_PLUGIN_DIR/$name" && git pull --quiet 2>&1)
      echo "  ✅ $name 已更新"
    else
      git clone --depth 1 "$url" "$ZSH_PLUGIN_DIR/$name" 2>&1 | tail -1
      echo "  ✅ $name 已安装"
    fi
  done
}
