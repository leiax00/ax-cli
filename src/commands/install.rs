use crate::config::{config_dir, expand_home, Config};
use crate::config::{TEMPLATE_BASHRC, TEMPLATE_ZSHRC};
use anyhow::Result;

pub fn execute(config: &Config, extras: bool) -> Result<()> {
    let cdir = config_dir();
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "0".into());
    let backup_dir = config_dir().join("backup").join(&ts);

    ensure_config_initialized(config)?;

    println!("🚀 开始部署开发环境...");
    println!("");
    println!(
        "🖥️  系统: {} ({})",
        crate::detect::os_name(),
        crate::detect::os_id()
    );
    println!("📦 包管理器: {}", crate::detect::pkg_manager());
    println!("📂 配置目录: {}", cdir.display());
    println!("");

    // 1. 安装系统包
    crate::packages::check_and_install(config, extras)?;

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

    // 6. 安装 shell 补全
    println!("");
    refresh_managed_shell_rcs(&cdir)?;
    println!("");
    install_shell_completions(config)?;

    // 7. 部署配置文件（从配置目录链接到系统位置）
    println!("");
    deploy_configs(config, &cdir, &backup_dir)?;
    println!("");
    install_shell_includes(&cdir, &backup_dir)?;

    println!("");
    println!("✅ 部署完成！");
    println!("📁 原有配置已备份到: {}", backup_dir.display());
    println!("");
    println!("👉 请重启终端，或运行: exec zsh");

    Ok(())
}

fn refresh_managed_shell_rcs(cdir: &std::path::Path) -> Result<()> {
    println!("📝 刷新托管 shell 配置...");

    let managed_files = [
        ("bash/.zshrc", TEMPLATE_ZSHRC),
        ("bash/.bashrc", TEMPLATE_BASHRC),
    ];

    for (rel, content) in managed_files {
        let path = cdir.join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, content)?;
        println!("  ✅ {}", rel);
    }

    Ok(())
}

fn install_shell_completions(config: &Config) -> Result<()> {
    println!("⌨️  安装 shell 补全...");

    for shell in ["zsh", "bash"] {
        match crate::commands::completion::execute(shell, false, config) {
            Ok(()) => {}
            Err(err) => println!("  ⚠️  {shell} 补全安装失败: {err}"),
        }
    }

    Ok(())
}

fn ensure_config_initialized(config: &Config) -> Result<()> {
    let cdir = config_dir();
    let config_file = cdir.join("config.yaml");

    if config_file.exists() {
        return Ok(());
    }

    println!("📁 未检测到 ax 配置，正在初始化默认配置...");
    crate::commands::config::init(false, config)?;
    println!("");
    Ok(())
}

const AX_ZSH_BLOCK: &str = r#"# >>> ax-cli >>>
[ -f "$HOME/.config/axconfig/bash/.zshrc" ] && source "$HOME/.config/axconfig/bash/.zshrc"
# <<< ax-cli <<<
"#;

const AX_BASH_BLOCK: &str = r#"# >>> ax-cli >>>
[ -f "$HOME/.config/axconfig/bash/.bashrc" ] && source "$HOME/.config/axconfig/bash/.bashrc"
# <<< ax-cli <<<
"#;

fn deploy_configs(
    config: &Config,
    cdir: &std::path::PathBuf,
    backup_dir: &std::path::Path,
) -> Result<()> {
    println!("🔗 链接配置文件...");

    for link in &config.deploy.links {
        let src = cdir.join(&link.src);
        let dst = expand_home(&link.dst);

        if is_shell_entrypoint(&dst) {
            println!(
                "  ⏭️  跳过入口文件: {} (改为通过 source 引入 ax 配置)",
                link.dst
            );
            continue;
        }

        if !src.exists() {
            if link.optional {
                continue;
            }
            println!("  ⚠️  源文件不存在: {} (跳过)", link.src);
            continue;
        }

        // 备份
        if dst.exists()
            && !std::fs::symlink_metadata(&dst)
                .map(|m| m.file_type().is_symlink())
                .unwrap_or(false)
        {
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

fn install_shell_includes(cdir: &std::path::Path, backup_dir: &std::path::Path) -> Result<()> {
    println!("🐚 安装 shell 引入配置...");
    ensure_shell_include(
        &expand_home("~/.zshrc"),
        &cdir.join("bash/.zshrc"),
        AX_ZSH_BLOCK,
        backup_dir,
    )?;
    ensure_shell_include(
        &expand_home("~/.bashrc"),
        &cdir.join("bash/.bashrc"),
        AX_BASH_BLOCK,
        backup_dir,
    )?;
    Ok(())
}

fn ensure_shell_include(
    shell_rc: &std::path::Path,
    managed_rc: &std::path::Path,
    block: &str,
    backup_dir: &std::path::Path,
) -> Result<()> {
    if !managed_rc.exists() {
        println!("  ⚠️  缺少配置片段: {} (跳过)", managed_rc.display());
        return Ok(());
    }

    let metadata = std::fs::symlink_metadata(shell_rc).ok();
    let is_symlink = metadata
        .as_ref()
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false);

    let mut content = if shell_rc.exists() && !is_symlink {
        std::fs::read_to_string(shell_rc)?
    } else {
        String::new()
    };

    if content.contains("# >>> ax-cli >>>") {
        println!("  ⏭️  已接入: {}", shell_rc.display());
        return Ok(());
    }

    if shell_rc.exists() && !is_symlink {
        backup_existing_file(shell_rc, backup_dir)?;
        println!("  📦 已备份: {}", shell_rc.display());
    }

    if is_symlink {
        let _ = std::fs::remove_file(shell_rc);
        println!("  🔄 已替换符号链接入口: {}", shell_rc.display());
    }

    if !content.is_empty() && !content.ends_with('\n') {
        content.push('\n');
    }
    if !content.is_empty() {
        content.push('\n');
    }
    content.push_str(block);

    if let Some(parent) = shell_rc.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(shell_rc, content)?;
    println!("  ✅ 已接入: {}", shell_rc.display());
    Ok(())
}

fn backup_existing_file(path: &std::path::Path, backup_dir: &std::path::Path) -> Result<()> {
    std::fs::create_dir_all(backup_dir)?;
    let rel = path.strip_prefix(expand_home("~")).unwrap_or(path);
    let backup_path = backup_dir.join(rel);
    if let Some(parent) = backup_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let _ = std::fs::copy(path, backup_path);
    Ok(())
}

fn is_shell_entrypoint(path: &std::path::Path) -> bool {
    let zshrc = expand_home("~/.zshrc");
    let bashrc = expand_home("~/.bashrc");
    path == zshrc || path == bashrc
}
