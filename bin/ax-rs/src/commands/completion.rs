use anyhow::Result;
use crate::config::Config;

const BASH_COMPLETION: &str = r#"# ax completion for bash
_ax_completions() {
    local cur="${COMP_WORDS[COMP_CWORD]}"
    local subcmd="${COMP_WORDS[1]}"

    if [ "$COMP_CWORD" -eq 1 ]; then
        COMPREPLY=($(compgen -W "init add edit list rm run sync pull update install proxy info help" -- "$cur"))
        return
    fi

    # ax proxy 子命令补全
    if [ "$COMP_CWORD" -eq 2 ] && [ "$subcmd" = "proxy" ]; then
        COMPREPLY=($(compgen -W "on off status" -- "$cur"))
        return
    fi

    # ax rm/edit 补全已注册命令名
    if [ "$COMP_CWORD" -eq 2 ] && { [ "$subcmd" = "rm" ] || [ "$subcmd" = "del" ] || [ "$subcmd" = "edit" ] || [ "$subcmd" = "run" ]; }; then
        local saved
        saved=$(ax list --quiet 2>/dev/null)
        COMPREPLY=($(compgen -W "$saved" -- "$cur"))
        return
    fi
}

complete -F _ax_completions ax"#;

const ZSH_COMPLETION: &str = r#"#compdef ax

_ax() {
    local -a commands subcommands
    commands=(
        'init:Initialize ax-cli'
        'add:Add a custom command'
        'edit:Edit an existing command'
        'list:List all commands'
        'rm:Remove a command'
        'run:Run a command'
        'sync:Sync commands to remote repo'
        'pull:Pull latest config from remote repo'
        'update:Update development environment'
        'install:Full installation'
        'info:Show current config and paths'
        'help:Print help'
    )

    if (( CURRENT == 2 )); then
        _describe 'command' commands
        return
    fi

    case "${words[1]}" in
        proxy)
            subcommands=('on:Turn proxy on' 'off:Turn proxy off' 'status:Show proxy status')
            if (( CURRENT == 3 )); then
                _describe 'action' subcommands
            fi
            ;;
        add|edit|rm|del|run)
            if (( CURRENT == 3 )); then
                local saved
                saved=(${(f)"$(ax list --quiet 2>/dev/null)"})
                _describe 'command' saved
            fi
            ;;
    esac
}

_ax"#;

const POWERSHELL_COMPLETION: &str = r#"
# ax PowerShell completion
Register-ArgumentCompleter -Native -CommandName ax -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)
    $commands = @('init', 'add', 'edit', 'list', 'rm', 'run', 'sync', 'pull', 'update', 'install', 'proxy', 'info', 'help')

    $prev = if ($commandAst.CommandElements.Count -ge 2) { $commandAst.CommandElements[-2].Value } else { '' }

    if ($prev -eq 'proxy') {
        @('on', 'off', 'status') | Where-Object { $_ -like "$wordToComplete*" }
    } else {
        $commands | Where-Object { $_ -like "$wordToComplete*" }
    }
}
"#;

pub fn execute(shell: &str, _config: &Config) -> Result<()> {
    let script = match shell {
        "bash" | "b" => BASH_COMPLETION,
        "zsh" | "z" => ZSH_COMPLETION,
        "powershell" | "pwsh" | "p" => POWERSHELL_COMPLETION,
        _ => {
            anyhow::bail!("不支持的 shell: {shell}\n支持: bash, zsh, powershell");
        }
    };

    println!("{script}");
    println!("# 将以上内容保存到对应位置：");
    println!("#   bash:     ~/.local/share/bash-completion/completions/ax");
    println!("#   zsh:      ~/.zsh/completions/_ax");
    println!("#   powershell: 将内容加入 $PROFILE");

    Ok(())
}
