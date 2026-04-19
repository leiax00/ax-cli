use anyhow::Result;
use crate::config::{Config, config_dir, expand_home};

pub fn execute(config: &Config) -> Result<()> {
    let local_dir = expand_home(&config.repo.local_dir);
    let remote = &config.repo.remote;

    println!("📦 拉取远程配置...");
    println!("   远程仓库: {}", remote);

    if local_dir.join(".git").exists() {
        // 已有本地仓库，git pull
        let output = std::process::Command::new("git")
            .args(["pull", "--quiet"])
            .current_dir(&local_dir)
            .output()?;
        if output.status.success() {
            println!("   ✅ 已更新本地仓库");
        } else {
            println!("   ⏭️  已是最新或更新失败");
            if !output.stderr.is_empty() {
                eprintln!("   {}", String::from_utf8_lossy(&output.stderr));
            }
        }
    } else {
        // clone
        println!("   克隆到: {}", local_dir.display());
        let output = std::process::Command::new("git")
            .args(["clone", "--quiet", remote, local_dir.to_str().unwrap()])
            .output()?;
        if output.status.success() {
            println!("   ✅ 克隆完成");
        } else {
            anyhow::bail!("克隆失败: {}", String::from_utf8_lossy(&output.stderr));
        }
    }

    // 同步仓库文件到配置目录
    let cdir = config_dir();
    let sync_items = vec![
        ("config.yaml", "config.yaml"),
        ("bash/.zshrc", "bash/.zshrc"),
        ("wezterm/wezterm.lua", "wezterm/wezterm.lua"),
    ];

    println!("");
    println!("📂 同步到配置目录...");
    for (src_rel, dst_rel) in &sync_items {
        let src = local_dir.join(src_rel);
        let dst = cdir.join(dst_rel);
        if src.exists() {
            if let Some(parent) = dst.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::copy(&src, &dst)?;
            println!("   ✅ {} → {}", src_rel, cdir.display());
        } else {
            println!("   ⏭️  {} 不存在于仓库", src_rel);
        }
    }

    // 同步 packages 目录
    let repo_pkgs = local_dir.join("packages");
    let config_pkgs = cdir.join("packages");
    if repo_pkgs.is_dir() {
        std::fs::create_dir_all(&config_pkgs)?;
        for entry in std::fs::read_dir(&repo_pkgs)? {
            let entry = entry?;
            let dst = config_pkgs.join(entry.file_name());
            std::fs::copy(entry.path(), &dst)?;
        }
        println!("   ✅ packages/ 同步完成");
    }

    println!("");
    println!("✅ 拉取完成！运行 ax install 或重启终端使配置生效。");
    Ok(())
}
