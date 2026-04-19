use anyhow::Result;
use crate::config::{Config, config_dir, expand_home};

pub fn execute(config: &Config) -> Result<()> {
    let cdir = config_dir();
    let cmd_file = expand_home(&config.ax.commands_file);

    println!("ax-cli 配置信息");
    println!("══════════════════════════════════════");
    println!("");
    println!("  配置目录:      {}", cdir.display());
    println!("  配置文件:      {}", cdir.join("config.yaml").display());
    println!("  配置片段目录:  {}", cdir.join("config.d").display());
    println!("  命令库:        {}", cmd_file.display());
    println!("  包列表目录:    {}", expand_home(&config.packages.dir).display());
    println!("");
    println!("  自动同步:      {}", if config.ax.auto_sync { "✅ 开启" } else { "❌ 关闭" });
    println!("  代理地址:      {}", config.proxy.address);
    println!("  Shell:         {}", config.shell.default);
    println!("  插件数:        {}", config.shell.plugins.len());
    println!("");
    println!("  Git 仓库:      {}", config.repo.remote);
    println!("  本地仓库:      {}", expand_home(&config.repo.local_dir).display());
    println!("");
    println!("  部署链接:");
    for link in &config.deploy.links {
        let opt = if link.optional { " (可选)" } else { "" };
        println!("    {} → {}{}", link.src, link.dst, opt);
    }

    Ok(())
}
