#!/usr/bin/env bash
# ============================================================
# 跨平台通知脚本
# 用法：./notify.sh [标题] [消息] [图标]
#
# 图标（Linux notify-send 支持）：
#   dialog-information / dialog-warning / dialog-error
# ============================================================

TITLE="${1:-Claude Code}"
MESSAGE="${2:-等待中...}"
ICON="${3:-dialog-information}"

# ---- 环境检测 -----------------------------------------------

is_wsl() {
  grep -qi microsoft /proc/version 2>/dev/null || [ -n "$WSL_DISTRO_NAME" ]
}

is_windows() {
  [ "$OS" = "Windows_NT" ]
}

is_linux() {
  [ "$(uname -s)" = "Linux" ]
}

# ---- 通知实现 -----------------------------------------------

notify_windows() {
  # 通过 Windows Forms 气泡通知（兼容 WSL 和原生 Windows Git Bash）
  # 转义单引号防止 PowerShell 命令注入
  local title_ps="${TITLE//\'/\'\'}"
  local message_ps="${MESSAGE//\'/\'\'}"
  powershell.exe -WindowStyle Hidden -Command "
    Add-Type -AssemblyName System.Windows.Forms
    \$n = New-Object System.Windows.Forms.NotifyIcon
    \$n.Icon = [System.Drawing.SystemIcons]::Information
    \$n.Visible = \$true
    \$n.ShowBalloonTip(4000, '$title_ps', '$message_ps', [System.Windows.Forms.ToolTipIcon]::Info)
    Start-Sleep 5
    \$n.Dispose()
  " >/dev/null 2>&1 &
}

notify_linux() {
  if command -v notify-send >/dev/null 2>&1; then
    notify-send --icon="$ICON" "$TITLE" "$MESSAGE"
  else
    echo "[通知] $TITLE: $MESSAGE" >&2
  fi
}

# ---- 分发 ---------------------------------------------------

if is_wsl; then
  notify_windows
elif is_windows; then
  notify_windows
elif is_linux; then
  notify_linux
fi

exit 0
