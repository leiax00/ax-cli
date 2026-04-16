# === PATH ===
export PATH="$HOME/.local/bin:$PATH"

# === 历史记录 ===
export HISTSIZE=50000
export HISTIGNORE="ls:ll:cd:pwd:clear:history"
shopt -s histappend
PROMPT_COMMAND="history -a; history -c; history -r; $PROMPT_COMMAND"

# === 自定义补全 ===
[ -f ~/.dotfiles/bash/completions/ax ] && source ~/.dotfiles/bash/completions/ax

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
