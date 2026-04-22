# === PATH ===
export PATH="$HOME/.local/bin:$PATH"

# === ax shell integration ===
ax() {
  if [ "$1" = "proxy" ] && { [ "$2" = "on" ] || [ "$2" = "off" ]; }; then
    eval "$(command ax "$@")"
  else
    command ax "$@"
  fi
}

# === 历史记录 ===
export HISTSIZE=50000
export HISTIGNORE="ls:ll:cd:pwd:clear:history"
shopt -s histappend
PROMPT_COMMAND="history -a; history -c; history -r; $PROMPT_COMMAND"

# === 自定义补全 ===
if [ -f /usr/share/bash-completion/bash_completion ]; then
  source /usr/share/bash-completion/bash_completion
elif [ -f /etc/bash_completion ]; then
  source /etc/bash_completion
elif [ -f "$HOME/.local/share/bash-completion/bash_completion" ]; then
  source "$HOME/.local/share/bash-completion/bash_completion"
fi

if [ -f "${XDG_DATA_HOME:-$HOME/.local/share}/bash-completion/completions/ax" ]; then
  source "${XDG_DATA_HOME:-$HOME/.local/share}/bash-completion/completions/ax"
fi

# === fzf ===
[ -f ~/.fzf.bash ] && source ~/.fzf.bash

# === Starship Prompt ===
if command -v starship &>/dev/null; then
  eval "$(starship init bash)"
fi

# === 常用 alias ===
alias ll='ls -alF'
alias la='ls -A'
alias l='ls -CF'
alias ..='cd ..'
alias ...='cd ../..'
alias grep='grep --color=auto'
alias egrep='egrep --color=auto'
alias fgrep='fgrep --color=auto'
alias cls='clear'

# === 自定义命令（ax add/edit/rm 自动更新）===
[ -f "$HOME/.config/axconfig/config.d/commands.sh" ] && source "$HOME/.config/axconfig/config.d/commands.sh"
