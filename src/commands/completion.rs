use crate::cli;
use crate::config::Config;
use anyhow::Result;
use clap_complete::{
    generate,
    shells::{Bash, PowerShell, Zsh},
};
use std::io::Write as _;

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
            let candidates = vec![
                dirs::home_dir().map(|h| h.join(".zsh").join("completions")),
                dirs::data_local_dir().map(|d| d.join("zsh").join("site-functions")),
            ];
            for candidate in candidates.into_iter().flatten() {
                if candidate.parent().map(|p| p.exists()).unwrap_or(false) {
                    return Ok(candidate);
                }
            }
            Ok(dirs::home_dir()
                .unwrap_or_default()
                .join(".zsh")
                .join("completions"))
        }
        _ => anyhow::bail!("不支持的 shell: {shell}\n支持: bash, zsh"),
    }
}

fn powershell_profile_targets(shell: &str) -> Vec<std::path::PathBuf> {
    let Some(documents_dir) = dirs::document_dir() else {
        return Vec::new();
    };
    powershell_profile_targets_in(&documents_dir, shell)
}

fn powershell_profile_targets_in(
    documents_dir: &std::path::Path,
    shell: &str,
) -> Vec<std::path::PathBuf> {
    let mut targets = Vec::new();

    if matches!(shell, "powershell" | "p") {
        targets.push(
            documents_dir
                .join("WindowsPowerShell")
                .join("Microsoft.PowerShell_profile.ps1"),
        );
    }

    if matches!(shell, "powershell" | "pwsh" | "p") {
        targets.push(
            documents_dir
                .join("PowerShell")
                .join("Microsoft.PowerShell_profile.ps1"),
        );
    }

    targets
}

fn build_script(shell: &str) -> Result<String> {
    let mut cmd = cli::command();
    let mut buf = Vec::new();

    match shell {
        "bash" | "b" => generate(Bash, &mut cmd, "ax", &mut buf),
        "zsh" | "z" => generate(Zsh, &mut cmd, "ax", &mut buf),
        "powershell" | "pwsh" | "p" => generate(PowerShell, &mut cmd, "ax", &mut buf),
        _ => anyhow::bail!("不支持的 shell: {shell}\n支持: bash, zsh, powershell, pwsh"),
    }

    let script = String::from_utf8(buf)?;

    match shell {
        "zsh" | "z" => Ok(post_process_zsh_script(&script)),
        _ => Ok(script),
    }
}

fn post_process_zsh_script(script: &str) -> String {
    let replacements = [
        (
            ":name -- 变量名:_default",
            ":name -- 变量名:_message -r \"变量名\"",
        ),
        (
            ":value -- 变量值:_default",
            ":value -- 变量值:_message -r \"变量值\"",
        ),
        (
            "-d+[变量描述]:DESC:_default",
            "-d+[变量描述]:DESC:_message -r \"变量描述\"",
        ),
        (
            "--desc=[变量描述]:DESC:_default",
            "--desc=[变量描述]:DESC:_message -r \"变量描述\"",
        ),
        (
            "-t+[标签，使用逗号分隔]:TAGS:_default",
            "-t+[标签，使用逗号分隔]:TAGS:_message -r \"标签，使用逗号分隔\"",
        ),
        (
            "--tags=[标签，使用逗号分隔]:TAGS:_default",
            "--tags=[标签，使用逗号分隔]:TAGS:_message -r \"标签，使用逗号分隔\"",
        ),
        (
            ":name -- 仅显示指定变量:_default",
            ":name -- 仅显示指定变量:_message -r \"变量名\"",
        ),
        (
            "-t+[按标签筛选]:TAG:_default",
            "-t+[按标签筛选]:TAG:_message -r \"标签\"",
        ),
        (
            "--tag=[按标签筛选]:TAG:_default",
            "--tag=[按标签筛选]:TAG:_message -r \"标签\"",
        ),
        (
            "(-a --all)-t+[暂停指定标签下的全部变量]:TAG:_default",
            "(-a --all)-t+[暂停指定标签下的全部变量]:TAG:_message -r \"标签\"",
        ),
        (
            "(-a --all)--tag=[暂停指定标签下的全部变量]:TAG:_default",
            "(-a --all)--tag=[暂停指定标签下的全部变量]:TAG:_message -r \"标签\"",
        ),
        (
            "(-a --all)-t+[恢复指定标签下的全部变量]:TAG:_default",
            "(-a --all)-t+[恢复指定标签下的全部变量]:TAG:_message -r \"标签\"",
        ),
        (
            "(-a --all)--tag=[恢复指定标签下的全部变量]:TAG:_default",
            "(-a --all)--tag=[恢复指定标签下的全部变量]:TAG:_message -r \"标签\"",
        ),
        (
            "*::names -- 一个或多个变量名:_default",
            "*::names -- 一个或多个变量名:_message -r \"变量名\"",
        ),
        (
            "-v+[新值]:VALUE:_default",
            "-v+[新值]:VALUE:_message -r \"新值\"",
        ),
        (
            "--value=[新值]:VALUE:_default",
            "--value=[新值]:VALUE:_message -r \"新值\"",
        ),
        (
            "-d+[新描述]:DESC:_default",
            "-d+[新描述]:DESC:_message -r \"新描述\"",
        ),
        (
            "--desc=[新描述]:DESC:_default",
            "--desc=[新描述]:DESC:_message -r \"新描述\"",
        ),
        (
            "-t+[新标签，使用逗号分隔]:TAGS:_default",
            "-t+[新标签，使用逗号分隔]:TAGS:_message -r \"新标签，使用逗号分隔\"",
        ),
        (
            "--tags=[新标签，使用逗号分隔]:TAGS:_default",
            "--tags=[新标签，使用逗号分隔]:TAGS:_message -r \"新标签，使用逗号分隔\"",
        ),
        (
            ":name -- 命令名称:_default",
            ":name -- 命令名称:_message -r \"命令名称\"",
        ),
        (
            ":cmd -- 要执行的命令内容:_default",
            ":cmd -- 要执行的命令内容:_message -r \"命令内容\"",
        ),
        (
            "::desc -- 命令描述:_default",
            "::desc -- 命令描述:_message -r \"命令描述\"",
        ),
        (
            "::name -- 命令名称，留空则进入交互选择:_default",
            "::name -- 命令名称，留空则进入交互选择:_message -r \"命令名称\"",
        ),
        (
            "::addr -- 代理地址，留空则使用配置中的地址:_default",
            "::addr -- 代理地址，留空则使用配置中的地址:_message -r \"代理地址\"",
        ),
        (
            ":shell -- 目标 shell：bash、zsh、powershell:_default",
            ":shell -- 目标 shell：bash、zsh、powershell:(bash zsh powershell)",
        ),
        (
            ":name -- Variable name:_default",
            ":name -- Variable name:_message -r \"Variable name\"",
        ),
        (
            ":value -- Variable value:_default",
            ":value -- Variable value:_message -r \"Variable value\"",
        ),
        (
            "-d+[Variable description]:DESC:_default",
            "-d+[Variable description]:DESC:_message -r \"Variable description\"",
        ),
        (
            "--desc=[Variable description]:DESC:_default",
            "--desc=[Variable description]:DESC:_message -r \"Variable description\"",
        ),
        (
            "-t+[Comma-separated tags]:TAGS:_default",
            "-t+[Comma-separated tags]:TAGS:_message -r \"Comma-separated tags\"",
        ),
        (
            "--tags=[Comma-separated tags]:TAGS:_default",
            "--tags=[Comma-separated tags]:TAGS:_message -r \"Comma-separated tags\"",
        ),
        (
            ":name -- Show only the specified variable:_default",
            ":name -- Show only the specified variable:_message -r \"Variable name\"",
        ),
        (
            "-t+[Filter by tag]:TAG:_default",
            "-t+[Filter by tag]:TAG:_message -r \"Tag\"",
        ),
        (
            "--tag=[Filter by tag]:TAG:_default",
            "--tag=[Filter by tag]:TAG:_message -r \"Tag\"",
        ),
        (
            "(-a --all)-t+[Pause all variables with the tag]:TAG:_default",
            "(-a --all)-t+[Pause all variables with the tag]:TAG:_message -r \"Tag\"",
        ),
        (
            "(-a --all)--tag=[Pause all variables with the tag]:TAG:_default",
            "(-a --all)--tag=[Pause all variables with the tag]:TAG:_message -r \"Tag\"",
        ),
        (
            "(-a --all)-t+[Resume all variables with the tag]:TAG:_default",
            "(-a --all)-t+[Resume all variables with the tag]:TAG:_message -r \"Tag\"",
        ),
        (
            "(-a --all)--tag=[Resume all variables with the tag]:TAG:_default",
            "(-a --all)--tag=[Resume all variables with the tag]:TAG:_message -r \"Tag\"",
        ),
        (
            "*::names -- One or more variable names:_default",
            "*::names -- One or more variable names:_message -r \"Variable name\"",
        ),
        (
            "-v+[New value]:VALUE:_default",
            "-v+[New value]:VALUE:_message -r \"New value\"",
        ),
        (
            "--value=[New value]:VALUE:_default",
            "--value=[New value]:VALUE:_message -r \"New value\"",
        ),
        (
            "-d+[New description]:DESC:_default",
            "-d+[New description]:DESC:_message -r \"New description\"",
        ),
        (
            "--desc=[New description]:DESC:_default",
            "--desc=[New description]:DESC:_message -r \"New description\"",
        ),
        (
            "-t+[New comma-separated tags]:TAGS:_default",
            "-t+[New comma-separated tags]:TAGS:_message -r \"New comma-separated tags\"",
        ),
        (
            "--tags=[New comma-separated tags]:TAGS:_default",
            "--tags=[New comma-separated tags]:TAGS:_message -r \"New comma-separated tags\"",
        ),
        (
            ":name -- Command name:_default",
            ":name -- Command name:_message -r \"Command name\"",
        ),
        (
            ":cmd -- Command body to execute:_default",
            ":cmd -- Command body to execute:_message -r \"Command body\"",
        ),
        (
            "::desc -- Command description:_default",
            "::desc -- Command description:_message -r \"Command description\"",
        ),
        (
            "::name -- Command name; omit to use interactive selection:_default",
            "::name -- Command name; omit to use interactive selection:_message -r \"Command name\"",
        ),
        (
            "::addr -- Proxy address; defaults to configured value:_default",
            "::addr -- Proxy address; defaults to configured value:_message -r \"Proxy address\"",
        ),
        (
            ":shell -- Target shell: bash, zsh, powershell:_default",
            ":shell -- Target shell: bash, zsh, powershell:(bash zsh powershell)",
        ),
    ];

    let mut out = script.to_string();
    for (from, to) in replacements {
        out = out.replace(from, to);
    }
    out
}

pub fn execute(shell: &str, print_only: bool, _config: &Config) -> Result<()> {
    let script = build_script(shell)?;

    if print_only {
        println!("{script}");
        return Ok(());
    }

    let shell_name = match shell {
        "bash" | "b" => "bash",
        "zsh" | "z" => "zsh",
        "powershell" | "pwsh" | "p" => "PowerShell",
        _ => shell,
    };

    if matches!(shell, "powershell" | "pwsh" | "p") {
        let targets = powershell_profile_targets(shell);
        if targets.is_empty() {
            anyhow::bail!("无法确定 PowerShell profile 路径");
        }

        let mut installed = Vec::new();
        for target in targets {
            let target_dir = target.parent().unwrap().to_path_buf();
            if target.exists() {
                let content = std::fs::read_to_string(&target)?;
                if content.contains("# powershell completion for ax") {
                    installed.push(target);
                    continue;
                }
            }
            std::fs::create_dir_all(target_dir)?;
            let mut f = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&target)?;
            f.write_all(b"\n\n# ax completion\n")?;
            f.write_all(script.as_bytes())?;
            installed.push(target);
        }

        println!("✅ {shell_name} 补全已安装到:");
        for target in installed {
            println!("   {}", target.display());
        }
        return Ok(());
    }

    let target = completion_dir(shell)?;
    let filename = match shell {
        "bash" | "b" => "ax",
        _ => "_ax",
    };
    let filepath = target.join(filename);

    std::fs::create_dir_all(&target)?;
    std::fs::write(&filepath, script)?;
    println!("✅ {shell_name} 补全已安装到: {}", filepath.display());
    println!("   重启终端或运行: source {}", filepath.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zsh_completion_includes_nested_commands_and_descriptions() {
        let script = build_script("zsh").unwrap();
        assert!(script.contains("config"));
        assert!(script.contains("env"));
        assert!(script.contains("proxy"));
        assert!(script.contains("环境变量管理"));
        assert!(script.contains(":name -- 变量名:_message -r \"变量名\""));
        assert!(script.contains(":value -- 变量值:_message -r \"变量值\""));
    }

    #[test]
    fn bash_completion_includes_nested_commands() {
        let script = build_script("bash").unwrap();
        assert!(script.contains("config"));
        assert!(script.contains("env"));
        assert!(script.contains("proxy"));
    }

    #[test]
    fn completion_paths_keep_shell_specific_directory() {
        let bash_dir = completion_dir("bash").unwrap();
        let zsh_dir = completion_dir("zsh").unwrap();

        assert!(bash_dir.ends_with("completions"));
        assert!(zsh_dir.ends_with("completions") || zsh_dir.ends_with("site-functions"));
    }

    #[test]
    fn powershell_installs_to_both_profiles_by_default() {
        let doc = std::path::PathBuf::from("/tmp/Documents");
        let targets = powershell_profile_targets_in(&doc, "powershell");

        assert_eq!(targets.len(), 2);
        assert!(targets[0].ends_with("WindowsPowerShell/Microsoft.PowerShell_profile.ps1"));
        assert!(targets[1].ends_with("PowerShell/Microsoft.PowerShell_profile.ps1"));
    }

    #[test]
    fn pwsh_installs_only_to_powershell_7_profile() {
        let doc = std::path::PathBuf::from("/tmp/Documents");
        let targets = powershell_profile_targets_in(&doc, "pwsh");

        assert_eq!(targets.len(), 1);
        assert!(targets[0].ends_with("PowerShell/Microsoft.PowerShell_profile.ps1"));
    }
}
