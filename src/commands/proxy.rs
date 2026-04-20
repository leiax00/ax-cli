use std::io::{self, IsTerminal};

use anyhow::Result;

use crate::config::Config;

#[derive(Debug, Clone, Copy, PartialEq)]
enum ShellType {
    Bash,
    PowerShell,
    Cmd,
}

pub fn execute(action: &crate::cli::ProxyAction, config: &Config) -> Result<()> {
    let shell_type = detect_shell_type();
    let interactive_stdout = io::stdout().is_terminal();

    match action {
        crate::cli::ProxyAction::On { addr } => {
            let proxy_addr = addr.as_deref().unwrap_or(&config.proxy.address);
            let no_proxy = &config.proxy.no_proxy;

            if interactive_stdout {
                print_apply_hint(shell_type, true, proxy_addr);
            } else {
                print!("{}", render_on_script(shell_type, proxy_addr, no_proxy));
            }
        }
        crate::cli::ProxyAction::Off => {
            if interactive_stdout {
                print_apply_hint(shell_type, false, "");
            } else {
                print!("{}", render_off_script(shell_type));
            }
        }
        crate::cli::ProxyAction::Status => {
            if let Ok(proxy) = std::env::var("http_proxy") {
                let no_proxy = std::env::var("no_proxy").unwrap_or_default();
                println!("🟢 Proxy: {proxy}");
                println!("   no_proxy: {no_proxy}");
            } else {
                println!("🔴 Proxy: OFF");
            }
        }
    }
    Ok(())
}

fn print_apply_hint(shell_type: ShellType, enable: bool, proxy_addr: &str) {
    let action = if enable { "on" } else { "off" };
    let command = match shell_type {
        ShellType::PowerShell => format!("Invoke-Expression \"$(ax proxy {action})\""),
        ShellType::Cmd => format!("for /f \"delims=\" %i in ('ax proxy {action}') do %i"),
        ShellType::Bash => format!("eval \"$(ax proxy {action})\""),
    };

    println!(
        "`ax proxy {action}` 只会输出当前 shell 需要执行的命令，不能直接修改父 shell 的环境变量。"
    );
    println!("请执行: {command}");
    if enable {
        println!("目标代理: {proxy_addr}");
    }
}

fn render_on_script(shell_type: ShellType, proxy_addr: &str, no_proxy: &str) -> String {
    match shell_type {
        ShellType::PowerShell => format!(
            "$env:http_proxy = \"{proxy}\"\n\
             $env:https_proxy = \"{proxy}\"\n\
             $env:all_proxy = \"{proxy}\"\n\
             $env:HTTP_PROXY = \"{proxy}\"\n\
             $env:HTTPS_PROXY = \"{proxy}\"\n\
             $env:ALL_PROXY = \"{proxy}\"\n\
             $env:no_proxy = \"{no_proxy}\"\n\
             $env:NO_PROXY = \"{no_proxy}\"\n\
             Write-Host \"🟢 Proxy ON: {proxy}\"\n",
            proxy = ps_escape(proxy_addr),
            no_proxy = ps_escape(no_proxy),
        ),
        ShellType::Cmd => format!(
            "set http_proxy={proxy}\n\
             set https_proxy={proxy}\n\
             set all_proxy={proxy}\n\
             set HTTP_PROXY={proxy}\n\
             set HTTPS_PROXY={proxy}\n\
             set ALL_PROXY={proxy}\n\
             set no_proxy={no_proxy}\n\
             set NO_PROXY={no_proxy}\n\
             echo 🟢 Proxy ON: {proxy}\n",
            proxy = proxy_addr,
            no_proxy = no_proxy,
        ),
        ShellType::Bash => format!(
            "export http_proxy={proxy}\n\
             export https_proxy={proxy}\n\
             export all_proxy={proxy}\n\
             export HTTP_PROXY={proxy}\n\
             export HTTPS_PROXY={proxy}\n\
             export ALL_PROXY={proxy}\n\
             export no_proxy={no_proxy}\n\
             export NO_PROXY={no_proxy}\n\
             echo \"🟢 Proxy ON: {display}\"\n",
            proxy = bash_escape(proxy_addr),
            no_proxy = bash_escape(no_proxy),
            display = proxy_addr,
        ),
    }
}

fn render_off_script(shell_type: ShellType) -> String {
    match shell_type {
        ShellType::PowerShell => "Remove-Item Env:http_proxy,Env:https_proxy,Env:all_proxy,Env:HTTP_PROXY,Env:HTTPS_PROXY,Env:ALL_PROXY,Env:no_proxy,Env:NO_PROXY -ErrorAction SilentlyContinue\nWrite-Host \"🔴 Proxy OFF\"\n".to_string(),
        ShellType::Cmd => "set http_proxy=\nset https_proxy=\nset all_proxy=\nset HTTP_PROXY=\nset HTTPS_PROXY=\nset ALL_PROXY=\nset no_proxy=\nset NO_PROXY=\necho 🔴 Proxy OFF\n".to_string(),
        ShellType::Bash => "unset http_proxy https_proxy all_proxy HTTP_PROXY HTTPS_PROXY ALL_PROXY no_proxy NO_PROXY\necho \"🔴 Proxy OFF\"\n".to_string(),
    }
}

fn detect_shell_type() -> ShellType {
    if let Ok(s) = std::env::var("AX_SHELL") {
        return match s.to_lowercase().as_str() {
            "powershell" | "pwsh" => ShellType::PowerShell,
            "cmd" | "cmd.exe" => ShellType::Cmd,
            _ => ShellType::Bash,
        };
    }

    let shell = std::env::var("SHELL").unwrap_or_default().to_lowercase();
    if shell.contains("pwsh") || shell.contains("powershell") {
        return ShellType::PowerShell;
    }
    if shell.contains("cmd.exe") {
        return ShellType::Cmd;
    }

    #[cfg(target_os = "windows")]
    {
        if std::env::var("PSModulePath").is_ok() {
            return ShellType::PowerShell;
        }
        return ShellType::Cmd;
    }

    ShellType::Bash
}

fn bash_escape(s: &str) -> String {
    let needs = s.contains(' ')
        || s.contains('\x27')
        || s.contains('"')
        || s.contains('$')
        || s.contains('\\');
    if needs {
        let escaped = s.replace("\x27", "'\\x27''");
        format!("'{}'", escaped)
    } else {
        s.to_string()
    }
}

fn ps_escape(s: &str) -> String {
    s.replace('"', "`\"")
}

#[cfg(test)]
mod tests {
    use super::{render_off_script, render_on_script, ShellType};

    #[test]
    fn renders_bash_proxy_exports() {
        let script = render_on_script(
            ShellType::Bash,
            "http://127.0.0.1:7890",
            "localhost,127.0.0.1",
        );
        assert!(script.contains("export http_proxy=http://127.0.0.1:7890"));
        assert!(script.contains("export NO_PROXY=localhost,127.0.0.1"));
    }

    #[test]
    fn renders_powershell_proxy_exports() {
        let script = render_on_script(ShellType::PowerShell, "http://127.0.0.1:7890", "localhost");
        assert!(script.contains("$env:http_proxy = \"http://127.0.0.1:7890\""));
        assert!(script.contains("$env:NO_PROXY = \"localhost\""));
    }

    #[test]
    fn renders_cmd_proxy_clear_commands() {
        let script = render_off_script(ShellType::Cmd);
        assert!(script.contains("set http_proxy="));
        assert!(script.contains("echo 🔴 Proxy OFF"));
    }
}
