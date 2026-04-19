use anyhow::Result;
use crate::config::Config;

pub fn execute(config: &Config) -> Result<()> {
    let _repo_dir = crate::expand(&config.ax.repo_dir);
    let backup_dir = format!("{}-{}", config.deploy.backup_dir, chrono_now());

    println!("🚀 开始部署开发环境...");
    println!("");
    println!("🖥️  系统: {} ({})", crate::detect::os_name(), crate::detect::os_id());
    println!("📦 包管理器: {}", crate::detect::pkg_manager());
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

    // 6. 部署配置文件
    println!("");
    deploy_dotfiles(config, &backup_dir)?;

    // 7. 部署 ax 工具
    println!("");
    deploy_ax_tool(config)?;

    println!("");
    println!("✅ 部署完成！");
    println!("📁 原有配置已备份到: {backup_dir}");
    println!("");
    println!("👉 请重启终端，或运行: exec zsh");

    Ok(())
}

fn deploy_dotfiles(config: &Config, backup_dir: &str) -> Result<()> {
    println!("🔗 链接配置文件...");
    let dotfiles_dir = crate::expand(&config.deploy.dotfiles_dir);
    let _home = crate::expand("~");

    for link in &config.deploy.links {
        let src = dotfiles_dir.join(&link.src);
        let dst = crate::expand(&link.dst);

        if !src.exists() && link.optional {
            continue;
        }

        // 备份
        if dst.exists() {
            if !std::fs::symlink_metadata(&dst).map(|m| m.file_type().is_symlink()).unwrap_or(false) {
                std::fs::create_dir_all(backup_dir)?;
                let backup_path = format!("{}/{}", backup_dir, link.dst.trim_start_matches("~/"));
                if let Some(parent) = std::path::Path::new(&backup_path).parent() {
                    std::fs::create_dir_all(parent)?;
                }
                let _ = std::fs::copy(&dst, &backup_path);
                println!("  📦 已备份: {}", link.dst);
            }
            let _ = std::fs::remove_file(&dst);
        }

        // 创建目标目录
        if let Some(parent) = dst.parent() {
            std::fs::create_dir_all(parent)?;
        }

        std::os::unix::fs::symlink(&src, &dst)?;
        println!("  ✅ {} → {}", link.src, link.dst);
    }

    Ok(())
}

fn deploy_ax_tool(config: &Config) -> Result<()> {
    println!("🔧 部署 ax 工具...");
    let repo_dir = crate::expand(&config.ax.repo_dir);
    let bin_dir = crate::expand("~/.local/bin");
    std::fs::create_dir_all(&bin_dir)?;

    let src_bin = repo_dir.join("bin").join("ax");
    let dst_bin = bin_dir.join("ax");
    let _ = std::fs::remove_file(&dst_bin);
    std::os::unix::fs::symlink(&src_bin, &dst_bin)?;
    println!("  ✅ ax");

    // 命令库符号链接
    let cmd_path = crate::expand(&config.ax.commands_file);
    let repo_cmd_path = repo_dir.join("ax-commands.json");
    if !cmd_path.exists() {
        std::os::unix::fs::symlink(&repo_cmd_path, &cmd_path)?;
        println!("  ✅ ax 命令库（符号链接）");
    } else {
        println!("  ⏭️  ax 命令库已存在");
    }

    Ok(())
}

fn chrono_now() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| {
            // 简单时间戳
            let secs = d.as_secs();
            format!("{}", secs)
        })
        .unwrap_or_else(|_| "unknown".into())
}
