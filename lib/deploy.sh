#!/bin/bash
# lib/deploy.sh - 配置文件部署
# 依赖: 无 (DOTDIR 由调用方设置)

backup() {
  [ -f "$HOME/$1" ] && mkdir -p "$BACKUP_DIR" && cp "$HOME/$1" "$BACKUP_DIR/" && echo "  📦 已备份: ~/$1"
  [ -d "$HOME/$1" ] && ! [ -L "$HOME/$1" ] && mkdir -p "$BACKUP_DIR" && cp -r "$HOME/$1" "$BACKUP_DIR/" && echo "  📦 已备份: ~/$1"
}

deploy_dotfiles() {
  echo ""
  echo "🔗 链接配置文件..."

  # zsh
  backup .zshrc
  ln -sf "$DOTDIR/bash/.zshrc" "$HOME/.zshrc"

  # wezterm
  backup .config/wezterm
  mkdir -p "$HOME/.config/wezterm"
  ln -sf "$DOTDIR/wezterm/wezterm.lua" "$HOME/.config/wezterm/wezterm.lua"

  # git
  [ -f "$HOME/.gitconfig" ] && backup .gitconfig
  [ -f "$DOTDIR/git/.gitconfig" ] && ln -sf "$DOTDIR/git/.gitconfig" "$HOME/.gitconfig"
}

deploy_ax_tool() {
  echo ""
  echo "🔧 部署 ax 工具..."
  mkdir -p "$HOME/.local/bin"
  chmod +x "$DOTDIR/bin/"*
  for tool in "$DOTDIR/bin/"*; do
    ln -sf "$tool" "$HOME/.local/bin/$(basename "$tool")"
    echo "  ✅ $(basename "$tool")"
  done

  # 命令库符号链接
  if [ ! -f "$HOME/.ax-commands.json" ]; then
    ln -sf "$DOTDIR/ax-commands.json" "$HOME/.ax-commands.json"
    echo "  ✅ ax 命令库已部署（符号链接）"
  else
    echo "  ⏭️  ax 命令库已存在"
  fi
}

update_ax_tool() {
  mkdir -p "$HOME/.local/bin"
  chmod +x "$DOTDIR/bin/"*
  for tool in "$DOTDIR/bin/"*; do
    ln -sf "$tool" "$HOME/.local/bin/$(basename "$tool")"
  done
}
