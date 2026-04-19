use anyhow::Result;
use crate::config::Config;

pub fn execute(action: &crate::cli::ProxyAction, config: &Config) -> Result<()> {
    match action {
        crate::cli::ProxyAction::On { addr } => {
            let proxy_addr = addr.as_deref().unwrap_or(&config.proxy.address);
            let no_proxy = &config.proxy.no_proxy;
            // 输出 shell 命令到 stdout，用 eval $(ax proxy on) 或 source <(ax proxy on)
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
        crate::cli::ProxyAction::Off => {
            println!("unset http_proxy https_proxy all_proxy HTTP_PROXY HTTPS_PROXY ALL_PROXY no_proxy NO_PROXY");
            println!("echo \"🔴 Proxy OFF\"");
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
