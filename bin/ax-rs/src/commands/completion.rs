use anyhow::Result;
use std::io::Write as _;
use crate::config::Config;

const BASH_COMPLETION: &str = r#"# ax completion for bash
_ax_completions() {
    local cur="${COMP_WORDS[COMP_CWORD]}"
    local subcmd="${COMP_WORDS[1]}"

    if [ "$COMP_CWORD" -eq 1 ]; then
        COMPREPLY=($(compgen -W "init add edit list rm run sync pull update install proxy completion info help" -- "$cur"))
        return
    fi

    if [ "$COMP_CWORD" -eq 2 ] && [ "$subcmd" = "proxy" ]; then
        COMPREPLY=($(compgen -W "on off status" -- "$cur"))
        return
    fi

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
        'completion:Generate shell completion'
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
        completion)
            subcommands=('bash:Bash completion' 'zsh:Zsh completion' 'powershell:PowerShell completion')
            if (( CURRENT == 3 )); then
                _describe 'shell' subcommands
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
    $commands = @('init', 'add', 'edit', 'list', 'rm', 'run', 'sync', 'pull', 'update', 'install', 'proxy', 'completion', 'info', 'help')

    $prev = if ($commandAst.CommandElements.Count -ge 2) { $commandAst.CommandElements[-2].Value } else { '' }

    if ($prev -eq 'proxy') {
        @('on', 'off', 'status') | Where-Object { $_ -like "$wordToComplete*" }
    } elseif ($prev -eq 'completion') {
        @('bash', 'zsh', 'powershell') | Where-Object { $_ -like "$wordToComplete*" }
    } else {
        $commands | Where-Object { $_ -like "$wordToComplete*" }
    }
}
"#;

fn completion_dir(shell: &str) -> Result<std::path::PathBuf> {
    match shell {
        "bash" | "b" => {
            let mut p = dirs::data_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("/usr/local/share"))
                .join("bash-completion")
                .join("completions");
            if !p.parent().unwrap().exists() {
                p = dirs::home_dir()
                    .unwrap_or_default()
                    .join(".local")
                    .join("share")
                    .join("bash-completion")
                    .join("completions");
            }
            Ok(p)
        }
        "zsh" | "z" => {
            // 尝试常见 zsh 补全目录
            let candidates = vec![
                dirs::home_dir().map(|h| h.join(".zsh").join("completions")),
                dirs::home_dir().map(|h| h.join(".oh-my-zsh").join("custom").join("plugins").join("ax")),
                dirs::data_local_dir().map(|d| d.join("zsh").join("site-functions")),
            ];
            for candidate in candidates.into_iter().flatten() {
                if candidate.parent().map(|p| p.exists()).unwrap_or(false) {
                    return Ok(candidate);
                }
            }
            // 默认创建 ~/.zsh/completions
            Ok(dirs::home_dir().unwrap_or_default().join(".zsh").join("completions"))
        }
        "powershell" | "pwsh" | "p" => {
            Ok(dirs::document_dir()
                .unwrap_or_default()
                .join("PowerShell")
                .join("Microsoft.PowerShell_profile.ps1"))
        }
        _ => anyhow::bail!("不支持的 shell: {shell}\n支持: bash, zsh, powershell"),
    }
}

pub fn execute(shell: &str, print_only: bool, config: &Config) -> Result<()> {
    let script = match shell {
        "bash" | "b" => BASH_COMPLETION,
        "zsh" | "z" => ZSH_COMPLETION,
        "powershell" | "pwsh" | "p" => POWERSHELL_COMPLETION,
        _ => anyhow::bail!("不支持的 shell: {shell}\n支持: bash, zsh, powershell"),
    };

    if print_only {
        println!("{script}");
        return Ok(());
    }

    // 检测当前 shell（如果用户没指定）
    let shell_name = match shell {
        "bash" | "b" => "bash",
        "zsh" | "z" => "zsh",
        "powershell" | "pwsh" | "p" => "PowerShell",
        _ => shell,
    };

    let target = completion_dir(shell)?;
    let target_dir = target.parent().unwrap().to_path_buf();

    // PowerShell 特殊处理（追加到 profile）
    if matches!(shell, "powershell" | "pwsh" | "p") {
        if target.exists() {
            let content = std::fs::read_to_string(&target)?;
            if content.contains("ax PowerShell completion") {
                println!("✅ {shell_name} 补全已安装: {}", target.display());
                return Ok(());
            }
        }
        std::fs::create_dir_all(target_dir)?;
        let mut f = std::fs::OpenOptions::new().create(true).append(true).open(&target)?;
        f.write_all(b"\n\n# ax completion\n")?;
        f.write_all(POWERSHELL_COMPLETION.as_bytes())?;
        println!("✅ {shell_name} 补全已安装到: {}", target.display());
        return Ok(());
    }

    // bash/zsh：写文件
    let filename = match shell {
        "bash" | "b" => "ax",
        _ => "_ax",
    };
    let filepath = target_dir.join(filename);

    std::fs::create_dir_all(&target_dir)?;
    std::fs::write(&filepath, script)?;
    println!("✅ {shell_name} 补全已安装到: {}", filepath.display());
    println!("   重启终端或运行: source {}", filepath.display());

    Ok(())
}
