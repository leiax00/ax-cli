use anyhow::Result;
use crate::config::Config;

pub fn execute(config: &Config) -> Result<()> {
    let repo_dir = crate::expand(&config.ax.repo_dir);

    if !repo_dir.join(".git").exists() {
        anyhow::bail!("❌ 未找到 dotfiles 仓库: {}", repo_dir.display());
    }

    println!("🔄 更新 ax-system-basic...");
    println!("");

    // 1. git pull
    println!("📦 拉取 dotfiles 仓库...");
    let output = std::process::Command::new("git")
        .args(["pull", "--quiet"])
        .current_dir(&repo_dir)
        .output()?;

    if output.status.success() {
        println!("  ✅ 已更新");
    } else {
        println!("  ⏭️  已是最新或更新失败");
    }

    // 2. 刷新 ax 工具链接
    println!("");
    println!("🔧 刷新 ax 工具链接...");
    let bin_dir = crate::expand("~/.local/bin");
    std::fs::create_dir_all(&bin_dir)?;
    let src_bin = repo_dir.join("bin").join("ax");
    let dst_bin = bin_dir.join("ax");
    let _ = std::fs::remove_file(&dst_bin);
    std::os::unix::fs::symlink(&src_bin, &dst_bin)?;
    println!("  ✅ ax");

    // 3. 检查系统包
    println!("");
    crate::packages::check_and_install(config)?;

    // 4. 更新 zsh 插件
    println!("");
    crate::shell::update_plugins(config)?;

    // 5. 检查字体
    println!("");
    crate::tools::check_font()?;

    println!("");
    println!("✅ 更新完成！运行 exec zsh 或重启终端使配置生效。");
    Ok(())
}
