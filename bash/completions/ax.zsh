# ax zsh 补全
_ax_completions() {
  local -a commands saved
  commands=('add:添加命令' 'edit:编辑命令' 'list:列出所有命令' 'ls:列出所有命令' 'rm:删除命令' 'del:删除命令' 'help:帮助')

  if (( CURRENT == 2 )); then
    saved=(${(f)"$(jq -r 'keys[]' ~/.ax-commands.json 2>/dev/null)"})
    _describe 'command' commands
    _describe 'saved' saved
    return
  fi

  local subcmd="${words[1]}"
  if [[ "$subcmd" == "rm" || "$subcmd" == "del" || "$subcmd" == "edit" ]]; then
    saved=(${(f)"$(jq -r 'keys[]' ~/.ax-commands.json 2>/dev/null)"})
    _describe 'saved' saved
    return
  fi
}

compdef _ax_completions ax
