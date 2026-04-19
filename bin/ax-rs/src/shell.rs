use anyhow::Result;
use crate::config::Config;

const PLUGIN_DIR: &str = "~/.zsh/plugins";

fn plugin_dir() -> std::path::PathBuf {
    crate::expand(PLUGIN_DIR)
}

pub fn install_zsh(_config: &Config) -> Result<()> {
    println!("🐚 安装 zsh...");
    if which::which("zsh").is_ok() {
        println!("  ⏭️  zsh 已安装");
    } else {
        match crate::detect::pkg_manager() {
            "apt" => {
                std::process::Command::new("sudo")
                    .args(["apt", "install", "-y", "-qq", "zsh"])
                    .status()?;
            }
            "dnf" => {
                std::process::Command::new("sudo")
                    .args(["dnf", "install", "-y", "zsh"])
                    .status()?;
            }
            "pacman" => {
                std::process::Command::new("sudo")
                    .args(["pacman", "-S", "--noconfirm", "zsh"])
                    .status()?;
            }
            "brew" => {
                std::process::Command::new("brew")
                    .args(["install", "zsh"])
                    .status()?;
            }
            _ => {}
        }
        println!("  ✅ zsh 安装完成");
    }
    Ok(())
}

pub fn set_default_shell() -> Result<()> {
    println!("🐚 设置 zsh 为默认 shell...");
    let zsh_path = which::which("zsh")
        .map(|p| p.display().to_string())
        .unwrap_or("/usr/bin/zsh".into());

    if std::env::var("SHELL").map(|s| s == zsh_path).unwrap_or(false) {
        println!("  ⏭️  zsh 已是默认 shell");
    } else {
        let _ = std::process::Command::new("chsh")
            .args(["-s", &zsh_path])
            .status();
        println!("  ✅ 默认 shell 已切换为 zsh（重启终端生效）");
    }
    Ok(())
}

pub fn install_plugins(config: &Config) -> Result<()> {
    println!("🔌 安装 zsh 插件...");
    let dir = plugin_dir();
    std::fs::create_dir_all(&dir)?;

    for plugin in &config.shell.plugins {
        let plugin_path = dir.join(&plugin.name);
        if plugin_path.exists() {
            println!("  ⏭️  {} 已存在", plugin.name);
        } else {
            let _ = std::process::Command::new("git")
                .args(["clone", "--depth", "1", &plugin.url, plugin_path.to_str().unwrap()])
                .output();
            println!("  ✅ {}", plugin.name);
        }
    }
    Ok(())
}

pub fn update_plugins(config: &Config) -> Result<()> {
    println!("🔌 更新 zsh 插件...");
    let dir = plugin_dir();
    std::fs::create_dir_all(&dir)?;

    for plugin in &config.shell.plugins {
        let plugin_path = dir.join(&plugin.name);
        if plugin_path.exists() {
            let _ = std::process::Command::new("git")
                .args(["pull", "--quiet"])
                .current_dir(&plugin_path)
                .output();
            println!("  ✅ {} 已更新", plugin.name);
        } else {
            let _ = std::process::Command::new("git")
                .args(["clone", "--depth", "1", &plugin.url, plugin_path.to_str().unwrap()])
                .output();
            println!("  ✅ {} 已安装", plugin.name);
        }
    }
    Ok(())
}
