#!/bin/bash
# install.sh - 一键部署开发环境
# 支持: Ubuntu/Debian, Fedora/RHEL, Arch/Manjaro
set -e

DOTDIR="$HOME/.dotfiles"
BACKUP_DIR="$HOME/.dotfiles-backup-$(date +%Y%m%d%H%M%S)"

# 加载模块
source "$DOTDIR/lib/detect.sh"
source "$DOTDIR/lib/packages.sh"
source "$DOTDIR/lib/shell.sh"
source "$DOTDIR/lib/tools.sh"
source "$DOTDIR/lib/deploy.sh"

echo "🚀 开始部署开发环境..."
log_os

# 执行部署
install_packages
install_zsh
set_default_shell
install_zsh_plugins
install_fzf
install_starship
install_nerd_font
deploy_dotfiles
deploy_ax_tool

echo ""
echo "✅ 部署完成！"
echo "📁 原有配置已备份到: $BACKUP_DIR"
echo ""
echo "👉 请重启终端，或运行: exec zsh"
