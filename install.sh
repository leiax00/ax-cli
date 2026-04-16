#!/bin/bash
# install.sh - 一键部署开发环境
set -e

DOTDIR="$HOME/.dotfiles"
BACKUP_DIR="$HOME/.dotfiles-backup-$(date +%Y%m%d%H%M%S)"

echo "🚀 开始部署开发环境..."

# 备份已有配置
backup() {
  [ -f "$HOME/$1" ] && mkdir -p "$BACKUP_DIR" && cp "$HOME/$1" "$BACKUP_DIR/" && echo "  📦 已备份: ~/$1"
  [ -d "$HOME/$1" ] && mkdir -p "$BACKUP_DIR" && cp -r "$HOME/$1" "$BACKUP_DIR/" && echo "  📦 已备份: ~/$1"
}

# 1. 安装 apt 包
echo ""
echo "📦 安装系统包..."
sudo apt update
sudo apt install -y $(cat "$DOTDIR/apt-packages.txt")

# 2. 安装字体
echo ""
echo "🔤 安装 Nerd Font..."
if ! fc-list | grep -q "JetBrains Mono"; then
  FONT_DIR="$HOME/.local/share/fonts"
  mkdir -p "$FONT_DIR"
  local_zip=$(mktemp)
  curl -fLo "$local_zip" https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/JetBrainsMono.zip
  unzip -qo "$local_zip" -d "$FONT_DIR"
  rm "$local_zip"
  fc-cache -fv
  echo "  ✅ 字体安装完成"
else
  echo "  ⏭️  字体已存在，跳过"
fi

# 3. 安装 fzf
echo ""
echo "🔍 检查 fzf..."
if ! command -v fzf &>/dev/null; then
  git clone --depth 1 https://github.com/junegunn/fzf.git ~/.fzf
  ~/.fzf/install --key-bindings --completion --no-update-rc
  echo "  ✅ fzf 安装完成"
else
  echo "  ⏭️  fzf 已存在，跳过"
fi

# 4. 安装 starship
echo ""
echo "🚀 检查 starship..."
if ! command -v starship &>/dev/null; then
  curl -sS https://starship.rs/install.sh | sh -s -- -y
  echo "  ✅ starship 安装完成"
else
  echo "  ⏭️  starship 已存在，跳过"
fi

# 5. 部署配置文件
echo ""
echo "🔗 链接配置文件..."
backup .bashrc
ln -sf "$DOTDIR/bash/.bashrc" "$HOME/.bashrc"

backup .config/wezterm
mkdir -p "$HOME/.config/wezterm"
ln -sf "$DOTDIR/wezterm/wezterm.lua" "$HOME/.config/wezterm/wezterm.lua"

[ -f "$HOME/.gitconfig" ] && backup .gitconfig
[ -f "$DOTDIR/git/.gitconfig" ] && ln -sf "$DOTDIR/git/.gitconfig" "$HOME/.gitconfig"

# 6. 部署自定义工具
echo ""
echo "🔧 部署自定义工具..."
mkdir -p "$HOME/.local/bin"
chmod +x "$DOTDIR/bin/"*
for tool in "$DOTDIR/bin/"*; do
  ln -sf "$tool" "$HOME/.local/bin/$(basename "$tool")"
  echo "  ✅ $(basename "$tool")"
done

# 7. 部署 ax 命令库（符号链接，修改直接写入仓库目录）
echo ""
if [ ! -f "$HOME/.ax-commands.json" ]; then
  ln -sf "$DOTDIR/ax-commands.json" "$HOME/.ax-commands.json"
  echo "  ✅ ax 命令库已部署（符号链接到仓库）"
else
  echo "  ⏭️  ax 命令库已存在，跳过"
fi

echo ""
echo "✅ 部署完成！"
echo "📁 原有配置已备份到: $BACKUP_DIR"
echo ""
echo "👉 请运行: source ~/.bashrc"
