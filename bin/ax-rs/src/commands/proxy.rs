use anyhow::Result;
use crate::config::Config;

fn is_powershell() -> bool {
    if let Ok(s) = std::env::var("AX_SHELL") {
        return matches!(s.to_lowercase().as_str(), "powershell" | "pwsh");
    }
    std::env::var("SHELL").map(|s| s.contains("pwsh") || s.contains("powershell")).unwrap_or(false)
        || std::env::var("PSModulePath").is_ok()
}

pub fn execute(action: &crate::cli::ProxyAction, config: &Config) -> Result<()> {
    match action {
        crate::cli::ProxyAction::On { addr } => {
            let proxy_addr = addr.as_deref().unwrap_or(&config.proxy.address);
            let no_proxy = &config.proxy.no_proxy;
            if is_powershell() {
                println!("$env:http_proxy = \"{proxy_addr}\"");
                println!("$env:https_proxy = \"{proxy_addr}\"");
                println!("$env:all_proxy = \"{proxy_addr}\"");
                println!("$env:HTTP_PROXY = \"{proxy_addr}\"");
                println!("$env:HTTPS_PROXY = \"{proxy_addr}\"");
                println!("$env:ALL_PROXY = \"{proxy_addr}\"");
                println!("$env:no_proxy = \"{no_proxy}\"");
                println!("$env:NO_PROXY = \"{no_proxy}\"");
                println!("Write-Host \"🟢 Proxy ON: {proxy_addr}\"");
            } else {
                println!("export http_proxy=\"{proxy_addr}\"");
                println!("export https_proxy=\"{proxy_addr}\"");
                println!("export all_proxy=\"{proxy_addr}\"");
                println!("export HTTP_PROXY=\"{proxy_addr}\"");
                println!("export HTTPS_PROXY=\"{proxy_addr}\"");
                println!("export ALL_PROXY=\"{proxy_addr}\"");
                println!("export no_proxy=\"{no_proxy}\"");
                println!("export NO_PROXY=\"{no_proxy}\"");
                println!("echo \"🟢 Proxy ON: {proxy_addr}\"");
            }
        }
        crate::cli::ProxyAction::Off => {
            if is_powershell() {
                println!("Remove-Item Env:http_proxy,Env:https_proxy,Env:all_proxy,Env:HTTP_PROXY,Env:HTTPS_PROXY,Env:ALL_PROXY,Env:no_proxy,Env:NO_PROXY -ErrorAction SilentlyContinue");
                println!("Write-Host \"🔴 Proxy OFF\"");
            } else {
                println!("unset http_proxy https_proxy all_proxy HTTP_PROXY HTTPS_PROXY ALL_PROXY no_proxy NO_PROXY");
                println!("echo \"🔴 Proxy OFF\"");
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
