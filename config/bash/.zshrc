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
setopt HIST_IGNORE_ALL_DUPS
setopt HIST_SAVE_NO_DUPS
setopt SHARE_HISTORY
setopt INC_APPEND_HISTORY

# === 插件（手动安装，不用 Oh My Zsh）===
# zsh-autosuggestions
if [ -f ~/.zsh/plugins/zsh-autosuggestions/zsh-autosuggestions.zsh ]; then
  source ~/.zsh/plugins/zsh-autosuggestions/zsh-autosuggestions.zsh
fi

# zsh-syntax-highlighting
if [ -f ~/.zsh/plugins/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh ]; then
  source ~/.zsh/plugins/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh
fi

# zsh-completions
if [ -d ~/.zsh/plugins/zsh-completions ]; then
  fpath=(~/.zsh/plugins/zsh-completions/src $fpath)
fi

# === 补全 ===
autoload -Uz compinit && compinit
zstyle ':completion:*' menu select
zstyle ':completion:*' matcher-list 'm:{a-zA-Z}={A-Za-z}'
setopt AUTO_LIST
setopt AUTO_MENU
setopt COMPLETE_IN_WORD

# === ax 补全 ===
[ -f ~/.ax/bash/completions/ax ] && source ~/.ax/bash/completions/ax

# === fzf ===
[ -f ~/.fzf.zsh ] && source ~/.fzf.zsh

# === Starship Prompt ===
if command -v starship &>/dev/null; then
  eval "$(starship init zsh)"
fi

# === 常用 alias ===
alias ll='ls -alF'
alias la='ls -A'
alias l='ls -CF'
alias ..='cd ..'
alias ...='cd ../..'
alias grep='grep --color=auto'
alias cls='clear'

# === 自定义命令（ax add/edit/rm 自动更新）===
[ -f "$HOME/.config/axconfig/config.d/commands.sh" ] && source "$HOME/.config/axconfig/config.d/commands.sh"

# === 键盘绑定 ===
bindkey -e
bindkey '^[[A' up-line-or-search      # 上下箭头搜索历史
bindkey '^[[B' down-line-or-search
bindkey '^[[Z' autosuggest-accept     # Shift+Tab 接受建议（可选）
