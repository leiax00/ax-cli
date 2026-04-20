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
        "powershell" | "pwsh" | "p" => Ok(dirs::document_dir()
            .unwrap_or_default()
            .join("PowerShell")
            .join("Microsoft.PowerShell_profile.ps1")),
        _ => anyhow::bail!("不支持的 shell: {shell}\n支持: bash, zsh, powershell"),
    }
}

fn build_script(shell: &str) -> Result<String> {
    let mut cmd = cli::command();
    let mut buf = Vec::new();

    match shell {
        "bash" | "b" => generate(Bash, &mut cmd, "ax", &mut buf),
        "zsh" | "z" => generate(Zsh, &mut cmd, "ax", &mut buf),
        "powershell" | "pwsh" | "p" => generate(PowerShell, &mut cmd, "ax", &mut buf),
        _ => anyhow::bail!("不支持的 shell: {shell}\n支持: bash, zsh, powershell"),
    }

    Ok(String::from_utf8(buf)?)
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

    let target = completion_dir(shell)?;

    if matches!(shell, "powershell" | "pwsh" | "p") {
        let target_dir = target.parent().unwrap().to_path_buf();
        if target.exists() {
            let content = std::fs::read_to_string(&target)?;
            if content.contains("# powershell completion for ax") {
                println!("✅ {shell_name} 补全已安装: {}", target.display());
                return Ok(());
            }
        }
        std::fs::create_dir_all(target_dir)?;
        let mut f = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&target)?;
        f.write_all(b"\n\n# ax completion\n")?;
        f.write_all(script.as_bytes())?;
        println!("✅ {shell_name} 补全已安装到: {}", target.display());
        return Ok(());
    }

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
}
