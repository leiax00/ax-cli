# === PATH ===
export PATH="$HOME/.local/bin:$PATH"

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
[ -f ~/.dotfiles/bash/completions/ax ] && source ~/.dotfiles/bash/completions/ax

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

# === Proxy ===
PROXY_ADDR="http://vpn.yushe.ai:7890"
NO_PROXY="localhost,127.0.0.1,10.0.0.0/8,172.16.0.0/12,192.168.0.0/16,*.local"

proxy_on() {
  export http_proxy="$PROXY_ADDR"
  export https_proxy="$PROXY_ADDR"
  export all_proxy="$PROXY_ADDR"
  export HTTP_PROXY="$PROXY_ADDR"
  export HTTPS_PROXY="$PROXY_ADDR"
  export ALL_PROXY="$PROXY_ADDR"
  export no_proxy="$NO_PROXY"
  export NO_PROXY="$NO_PROXY"
  echo "🟢 Proxy ON: $PROXY_ADDR"
}

proxy_off() {
  unset http_proxy https_proxy all_proxy HTTP_PROXY HTTPS_PROXY ALL_PROXY no_proxy NO_PROXY
  echo "🔴 Proxy OFF"
}

proxy_status() {
  if [ -n "$http_proxy" ]; then
    echo "🟢 Proxy: $http_proxy"
    echo "   no_proxy: $no_proxy"
  else
    echo "🔴 Proxy: OFF"
  fi
}

alias pn='proxy_on'
alias pf='proxy_off'
alias ps='proxy_status'

# === 键盘绑定 ===
bindkey -e
bindkey '^[[A' up-line-or-search      # 上下箭头搜索历史
bindkey '^[[B' down-line-or-search
bindkey '^[[Z' autosuggest-accept     # Shift+Tab 接受建议（可选）
