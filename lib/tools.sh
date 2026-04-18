#!/bin/bash
# lib/tools.sh - fzf, starship, 字体等通用工具
# 依赖: 无

install_fzf() {
  echo ""
  echo "🔍 检查 fzf..."
  if command -v fzf &>/dev/null; then
    echo "  ⏭️  fzf 已存在"
  else
    git clone --depth 1 https://github.com/junegunn/fzf.git ~/.fzf
    ~/.fzf/install --key-bindings --completion --no-update-rc
    echo "  ✅ fzf 安装完成"
  fi
}

install_starship() {
  echo ""
  echo "🚀 检查 starship..."
  if command -v starship &>/dev/null; then
    echo "  ⏭️  starship 已存在"
  else
    curl -sS https://starship.rs/install.sh | sh -s -- -y
    echo "  ✅ starship 安装完成"
  fi
}

install_nerd_font() {
  echo ""
  echo "🔤 检查 Nerd Font..."
  if fc-list 2>/dev/null | grep -q "JetBrains Mono"; then
    echo "  ⏭️  字体已存在"
  else
    local FONT_DIR="$HOME/.local/share/fonts"
    mkdir -p "$FONT_DIR"
    local tmp_zip=$(mktemp)
    curl -fLo "$tmp_zip" https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/JetBrainsMono.zip
    unzip -qo "$tmp_zip" -d "$FONT_DIR"
    rm "$tmp_zip"
    fc-cache -fv 2>/dev/null
    echo "  ✅ JetBrains Mono Nerd Font 安装完成"
  fi
}
