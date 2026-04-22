use anyhow::Result;
use crate::config::{Config, expand_home};

pub fn execute(config: &Config) -> Result<()> {
    println!("🔄 更新 ax-cli...");
    println!("");

    // 1. git pull 仓库
    let local_dir = expand_home(&config.repo.local_dir);
    if local_dir.join(".git").exists() {
        println!("📦 拉取远程仓库...");
        let _ = std::process::Command::new("git")
            .args(["pull", "--quiet"])
            .current_dir(&local_dir)
            .output();
        println!("  ✅ 已更新");
    }

    // 2. 同步仓库配置到本地
    println!("");
    crate::commands::pull::execute(config)?;

    // 3. 检查系统包
    println!("");
    crate::packages::check_and_install(config, false)?;

    // 4. 更新 zsh 插件
    println!("");
    crate::shell::update_plugins(config)?;

    // 5. 检查字体
    println!("");
    crate::tools::check_font()?;

    println!("");
    println!("✅ 更新完成！重启终端，或重新加载对应的 shell 入口文件使配置生效。");
    Ok(())
}
