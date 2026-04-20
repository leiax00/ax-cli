use crate::config::{config_dir, load_all_commands, Config};
use anyhow::Result;

pub fn execute(config: &Config) -> Result<()> {
    let cdir = config_dir();

    println!("ax-cli 配置信息");
    println!("══════════════════════════════════════");
    println!("");
    println!("  配置目录:      {}", cdir.display());
    println!("  配置文件:      {}", cdir.join("config.yaml").display());
    println!("  配置片段目录:  {}", cdir.join("config.d").display());
    println!(
        "  命令文件:      {}",
        cdir.join("config.d/commands.yaml").display()
    );
    println!("  包列表目录:    {}", cdir.join("packages").display());
    println!("");
    println!(
        "  自动同步:      {}",
        if config.ax.auto_sync {
            "✅ 开启"
        } else {
            "❌ 关闭"
        }
    );
    println!("  代理地址:      {}", config.proxy.address);
    println!("  Shell:         {}", config.shell.default);
    println!("  插件数:        {}", config.shell.plugins.len());
    println!("  已注册命令:    {}", load_all_commands(config)?.len());

    // 显示 git remote
    if cdir.join(".git").exists() {
        if let Ok(output) = std::process::Command::new("git")
            .args(["remote", "get-url", "origin"])
            .current_dir(&cdir)
            .output()
        {
            let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !url.is_empty() {
                println!("  远程仓库:      {}", url);
            }
        }
    }

    println!("");
    println!("  部署链接:");
    for link in &config.deploy.links {
        let opt = if link.optional { " (可选)" } else { "" };
        println!("    {} → {}{}", link.src, link.dst, opt);
    }
    println!("  Shell 引入:");
    println!("    ~/.zshrc  -> source ~/.config/ax-cli/bash/.zshrc");
    println!("    ~/.bashrc -> source ~/.config/ax-cli/bash/.bashrc");

    Ok(())
}
