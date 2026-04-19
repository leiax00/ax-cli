use anyhow::Result;
use crate::config::{Config, config_dir, expand_home};

pub fn execute(config: &Config) -> Result<()> {
    let cdir = config_dir();
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "0".into());
    let backup_dir = expand_home(&format!("~/.ax-backup-{}", ts));

    println!("🚀 开始部署开发环境...");
    println!("");
    println!("🖥️  系统: {} ({})", crate::detect::os_name(), crate::detect::os_id());
    println!("📦 包管理器: {}", crate::detect::pkg_manager());
    println!("📂 配置目录: {}", cdir.display());
    println!("");

    // 1. 安装系统包
    crate::packages::check_and_install(config)?;

    // 2. 安装 zsh
    println!("");
    crate::shell::install_zsh(config)?;

    // 3. 设置默认 shell
    println!("");
    crate::shell::set_default_shell()?;

    // 4. 安装 zsh 插件
    println!("");
    crate::shell::install_plugins(config)?;

    // 5. 安装工具
    println!("");
    crate::tools::install_fzf()?;
    println!("");
    crate::tools::install_starship()?;
    println!("");
    crate::tools::check_font()?;

    // 6. 部署配置文件（从配置目录链接到系统位置）
    println!("");
    deploy_configs(config, &cdir, &backup_dir)?;

    println!("");
    println!("✅ 部署完成！");
    println!("📁 原有配置已备份到: {}", backup_dir.display());
    println!("");
    println!("👉 请重启终端，或运行: exec zsh");

    Ok(())
}

fn deploy_configs(config: &Config, cdir: &std::path::PathBuf, backup_dir: &std::path::Path) -> Result<()> {
    println!("🔗 链接配置文件...");

    for link in &config.deploy.links {
        let src = cdir.join(&link.src);
        let dst = expand_home(&link.dst);

        if !src.exists() {
            if link.optional {
                continue;
            }
            println!("  ⚠️  源文件不存在: {} (跳过)", link.src);
            continue;
        }

        // 备份
        if dst.exists() && !std::fs::symlink_metadata(&dst).map(|m| m.file_type().is_symlink()).unwrap_or(false) {
            std::fs::create_dir_all(backup_dir)?;
            let rel = link.dst.trim_start_matches("~/");
            let backup_path = backup_dir.join(rel);
            if let Some(parent) = backup_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let _ = std::fs::copy(&dst, &backup_path);
            println!("  📦 已备份: {}", link.dst);
        }

        // 移除已有链接
        let _ = std::fs::remove_file(&dst);

        // 创建目标目录
        if let Some(parent) = dst.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // 创建符号链接
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&src, &dst)?;
        }
        #[cfg(windows)]
        {
            std::os::windows::fs::symlink_file(&src, &dst)?;
        }

        println!("  ✅ {} → {}", link.src, link.dst);
    }

    Ok(())
}
