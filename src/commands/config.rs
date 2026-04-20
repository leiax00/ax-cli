use crate::config::{
    config_dir, expand_home, Config, TEMPLATE_BASHRC, TEMPLATE_CONFIG_YAML, TEMPLATE_WEZTERM,
    TEMPLATE_ZSHRC,
};
use anyhow::{Context, Result};

pub fn init(force: bool, _config: &Config) -> Result<()> {
    let cdir = config_dir();

    if cdir.exists() && !force {
        println!("⚠️  配置目录已存在: {}", cdir.display());
        println!("   使用 ax config init --force 强制覆盖同名文件");
        return Ok(());
    }

    println!("🚀 初始化 ax-cli 配置...");
    println!("   配置目录: {}", cdir.display());

    // 创建目录结构
    for dir in ["config.d", "bash", "wezterm", "packages", "git"] {
        std::fs::create_dir_all(cdir.join(dir))?;
    }
    println!("   ✅ 目录结构");

    // 写入默认配置
    let files = vec![
        ("config.yaml", TEMPLATE_CONFIG_YAML),
        ("bash/.bashrc", TEMPLATE_BASHRC),
        ("bash/.zshrc", TEMPLATE_ZSHRC),
        ("wezterm/wezterm.lua", TEMPLATE_WEZTERM),
    ];
    for (rel, content) in &files {
        let path = cdir.join(rel);
        if !path.exists() || force {
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&path, *content)?;
            println!("   ✅ {}", rel);
        } else {
            println!("   ⏭️  {} (已存在)", rel);
        }
    }

    // 写入当前系统的包列表
    let pkg_file = cdir
        .join("packages")
        .join(format!("{}.txt", crate::detect::os_id()));
    if !pkg_file.exists() {
        let default_pkgs = match crate::detect::os_id().as_str() {
            "ubuntu" | "debian" => include_str!("../../config/packages/ubuntu.txt"),
            "fedora" | "rhel" | "centos" => include_str!("../../config/packages/fedora.txt"),
            "arch" | "manjaro" => include_str!("../../config/packages/arch.txt"),
            _ => "[core]\n# Git for plugin and repo operations\ngit\n# curl for downloads and installer scripts\ncurl\n# zsh as the default managed shell\nzsh\n# unzip for Nerd Font archives\nunzip\n# fontconfig provides fc-list and fc-cache\nfontconfig\n\n[extras]\n# jq for JSON filtering in scripts\njq\n# fzf for fuzzy search and pickers\nfzf\n# wget as an additional download tool\nwget\n# tree for directory tree display\ntree\n# htop for interactive process monitoring\nhtop\n# ripgrep for fast text search\nripgrep\n# bat for syntax-highlighted file viewing\nbat\n# tmux for terminal multiplexing\ntmux\n",
        };
        std::fs::write(&pkg_file, default_pkgs)?;
        println!("   ✅ packages/{}.txt", crate::detect::os_id());
    } else {
        println!("   ⏭️  packages/{}.txt (已存在)", crate::detect::os_id());
    }

    // git init
    let git_dir = cdir.join(".git");
    if !git_dir.exists() {
        let _ = std::process::Command::new("git")
            .args(["init", "-b", "main"])
            .current_dir(&cdir)
            .output();
        // 创建 .gitignore
        std::fs::write(cdir.join(".gitignore"), "*.log\n*.tmp\n.DS_Store\n")?;
        println!("   ✅ git 仓库初始化");
    } else {
        println!("   ⏭️  git 仓库已存在");
    }

    println!("");
    println!("✅ 初始化完成！");
    println!("");
    println!("下一步：");
    println!("  ax config remote <url>    # 设置远程仓库（可选）");
    println!("  ax install                # 安装系统包、工具、部署配置");
    println!("  ax info                   # 查看配置");

    Ok(())
}

pub fn remote(url: Option<&str>, _config: &Config) -> Result<()> {
    let cdir = config_dir();
    let git_dir = cdir.join(".git");

    if !git_dir.exists() {
        anyhow::bail!("配置目录未初始化，请先运行 ax config init");
    }

    match url {
        Some(u) => {
            // 检查是否已有 remote
            let output = std::process::Command::new("git")
                .args(["remote", "get-url", "origin"])
                .current_dir(&cdir)
                .output();

            if output.is_ok() && !output.unwrap().stdout.is_empty() {
                // 更新
                let _ = std::process::Command::new("git")
                    .args(["remote", "set-url", "origin", u])
                    .current_dir(&cdir)
                    .output();
                println!("✅ 远程仓库已更新: {u}");
            } else {
                // 添加
                let _ = std::process::Command::new("git")
                    .args(["remote", "add", "origin", u])
                    .current_dir(&cdir)
                    .output();
                println!("✅ 远程仓库已设置: {u}");
            }
        }
        None => {
            let output = std::process::Command::new("git")
                .args(["remote", "get-url", "origin"])
                .current_dir(&cdir)
                .output()?;
            if output.stdout.is_empty() {
                println!("未设置远程仓库");
            } else {
                println!("{}", String::from_utf8_lossy(&output.stdout).trim());
            }
        }
    }

    Ok(())
}

pub fn push(_config: &Config) -> Result<()> {
    let cdir = config_dir();
    let git_dir = cdir.join(".git");

    if !git_dir.exists() {
        anyhow::bail!("配置目录未初始化，请先运行 ax config init");
    }

    // git add
    let add_output = std::process::Command::new("git")
        .args(["add", "-A"])
        .current_dir(&cdir)
        .output()
        .context("git add 失败")?;
    if !add_output.status.success() {
        anyhow::bail!(
            "git add 失败: {}",
            String::from_utf8_lossy(&add_output.stderr)
        );
    }

    // 检查是否有暂存变更，有则提交
    let has_changes = !std::process::Command::new("git")
        .args(["diff", "--cached", "--quiet"])
        .current_dir(&cdir)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    if has_changes {
        let commit_output = std::process::Command::new("git")
            .args(["commit", "-m", "sync: ax-cli config"])
            .current_dir(&cdir)
            .output();

        if commit_output.is_err() || !commit_output.as_ref().unwrap().status.success() {
            let stderr = match &commit_output {
                Err(e) => e.to_string(),
                Ok(o) => String::from_utf8_lossy(&o.stderr).to_string(),
            };
            anyhow::bail!("提交失败: {stderr}");
        }
    }

    // 检查是否有未推送的提交
    let has_unpushed = std::process::Command::new("git")
        .args(["log", "@{u}..HEAD", "--oneline"])
        .stderr(std::process::Stdio::null())
        .current_dir(&cdir)
        .output()
        .map(|o| !String::from_utf8_lossy(&o.stdout).trim().is_empty())
        .unwrap_or(false);

    // 无新提交且无未推送的提交
    if !has_changes && !has_unpushed {
        println!("⏭️  没有变更需要推送");
        return Ok(());
    }

    // 获取当前分支名
    let branch_output = std::process::Command::new("git")
        .args(["branch", "--show-current"])
        .current_dir(&cdir)
        .output()
        .context("获取当前分支失败")?;
    let branch = String::from_utf8_lossy(&branch_output.stdout).trim().to_string();

    // 检查是否有 upstream
    let has_upstream = std::process::Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "@{u}"])
        .stderr(std::process::Stdio::null())
        .current_dir(&cdir)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    let push_args = if has_upstream {
        vec!["push", "origin", "HEAD"]
    } else {
        vec!["push", "-u", "origin", &branch]
    };

    let output = std::process::Command::new("git")
        .args(&push_args)
        .current_dir(&cdir)
        .output();

    if output.is_ok() && output.as_ref().unwrap().status.success() {
        println!("☁️  已推送到远程仓库");
    } else {
        let o = output.unwrap();
        let stderr = String::from_utf8_lossy(&o.stderr);
        if stderr.contains("no remote") {
            println!("⚠️  推送失败，请先设置远程仓库：");
            println!("   ax config remote <url>");
        } else {
            println!("⚠️  推送失败: {stderr}");
        }
    }

    Ok(())
}

pub fn pull(_config: &Config) -> Result<()> {
    let cdir = config_dir();
    let git_dir = cdir.join(".git");

    if !git_dir.exists() {
        anyhow::bail!("配置目录未初始化，请先运行 ax config init");
    }

    let output = std::process::Command::new("git")
        .args(["pull", "--rebase", "--quiet"])
        .current_dir(&cdir)
        .output()?;

    if output.status.success() {
        println!("📦 配置已更新");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("no remote") || stderr.contains("fatal") {
            println!("⚠️  拉取失败，可能需要先设置远程仓库：");
            println!("   ax config remote <url>");
        } else {
            println!("📦 已是最新或拉取失败");
        }
    }

    Ok(())
}

pub fn export(with_binary: bool, output: Option<&str>, _config: &Config) -> Result<()> {
    let cdir = config_dir();
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "0".into());

    let filename = output
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("ax-cli-config-{}.tar.gz", ts));

    let output_path = if filename.starts_with('/') || filename.starts_with('~') {
        expand_home(&filename)
    } else {
        std::env::current_dir().unwrap_or_default().join(&filename)
    };

    println!("📦 导出配置到: {}", output_path.display());

    // 用 git archive 或 tar 打包
    if cdir.join(".git").exists() {
        // 如果是 git 仓库，用 git archive
        let _ = std::process::Command::new("git")
            .args(["add", "-A"])
            .current_dir(&cdir)
            .output();

        let output_file = output_path.to_str().unwrap();
        let result = std::process::Command::new("sh")
            .args([
                "-c",
                &format!(
                    "cd '{}' && git archive --format=tar.gz HEAD -o '{}' {}",
                    cdir.display(),
                    output_file,
                    if with_binary { "" } else { " -- . ':!ax'" }
                ),
            ])
            .output();

        match result {
            Ok(o) if o.status.success() => {}
            _ => {
                // fallback: tar
                tar_export(&cdir, &output_path, with_binary)?;
            }
        }
    } else {
        tar_export(&cdir, &output_path, with_binary)?;
    }

    if with_binary {
        println!("   ✅ 包含 ax 二进制");
    }

    // 检查文件大小
    if let Ok(meta) = std::fs::metadata(&output_path) {
        println!("   ✅ 文件大小: {} bytes", meta.len());
    }

    println!("   ✅ 导出完成");
    Ok(())
}

fn tar_export(
    cdir: &std::path::Path,
    output_path: &std::path::Path,
    with_binary: bool,
) -> Result<()> {
    let tmp_dir = std::env::temp_dir().join("ax-export");
    let _ = std::fs::remove_dir_all(&tmp_dir);
    std::fs::create_dir_all(&tmp_dir)?;

    // 复制配置文件（排除 .git）
    copy_dir_exclude(cdir, &tmp_dir, &[".git"])?;

    // 如果包含二进制
    if with_binary {
        if let Ok(exe) = std::env::current_exe() {
            let bin_dir = tmp_dir.join("bin");
            std::fs::create_dir_all(&bin_dir)?;
            let dest = bin_dir.join(if cfg!(windows) { "ax.exe" } else { "ax" });
            std::fs::copy(&exe, &dest)?;
        }
    }

    // 打包
    let output_file = output_path.to_str().context("invalid path")?;
    let _ = std::process::Command::new("tar")
        .args(["-czf", output_file, "-C", tmp_dir.to_str().unwrap(), "."])
        .output();

    let _ = std::fs::remove_dir_all(&tmp_dir);
    Ok(())
}

fn copy_dir_exclude(src: &std::path::Path, dst: &std::path::Path, exclude: &[&str]) -> Result<()> {
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let name = entry.file_name();
        if exclude.iter().any(|e| name == *e) {
            continue;
        }
        let src_path = entry.path();
        let dst_path = dst.join(&name);
        if src_path.is_dir() {
            std::fs::create_dir_all(&dst_path)?;
            copy_dir_exclude(&src_path, &dst_path, exclude)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

pub fn import(file: &str, _config: &Config) -> Result<()> {
    let path = expand_home(file);
    if !path.exists() {
        anyhow::bail!("文件不存在: {}", path.display());
    }

    let cdir = config_dir();

    println!("📥 导入配置从: {}", path.display());
    println!("   目标: {}", cdir.display());

    let tmp_dir = std::env::temp_dir().join("ax-import");
    let _ = std::fs::remove_dir_all(&tmp_dir);
    std::fs::create_dir_all(&tmp_dir)?;

    // 解压
    let _ = std::process::Command::new("tar")
        .args([
            "-xzf",
            path.to_str().unwrap(),
            "-C",
            tmp_dir.to_str().unwrap(),
        ])
        .output();

    // 复制到配置目录（保留已有配置不覆盖）
    let mut count = 0u32;
    for entry in std::fs::read_dir(&tmp_dir)? {
        let entry = entry?;
        let name = entry.file_name();
        if name == "bin" {
            continue; // 不导入 bin，ax 自身管理
        }
        let src_path = entry.path();
        let dst_path = cdir.join(&name);
        if src_path.is_dir() {
            copy_dir_merge(&src_path, &dst_path, &mut count)?;
        } else {
            if !dst_path.exists() {
                if let Some(parent) = dst_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::copy(&src_path, &dst_path)?;
                count += 1;
            }
        }
    }

    // 如果有 bin/ax，复制到 ~/.local/bin
    let imported_bin = tmp_dir
        .join("bin")
        .join(if cfg!(windows) { "ax.exe" } else { "ax" });
    if imported_bin.exists() {
        let local_bin = expand_home("~/.local/bin");
        std::fs::create_dir_all(&local_bin)?;
        let dest = local_bin.join(if cfg!(windows) { "ax.exe" } else { "ax" });
        std::fs::copy(&imported_bin, &dest)?;
        #[cfg(unix)]
        {
            let _ = std::process::Command::new("chmod")
                .args(["+x", dest.to_str().unwrap()])
                .output();
        }
        println!("   ✅ ax 二进制 → {}", dest.display());
    }

    let _ = std::fs::remove_dir_all(&tmp_dir);

    println!("   ✅ 导入了 {count} 个文件");
    println!("");
    println!("✅ 导入完成！运行 ax info 查看");

    Ok(())
}

fn copy_dir_merge(src: &std::path::Path, dst: &std::path::Path, count: &mut u32) -> Result<()> {
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            if !dst_path.exists() {
                std::fs::create_dir_all(&dst_path)?;
            }
            copy_dir_merge(&src_path, &dst_path, count)?;
        } else if !dst_path.exists() {
            if let Some(parent) = dst_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::copy(&src_path, &dst_path)?;
            *count += 1;
        }
    }
    Ok(())
}

pub fn path(_config: &Config) -> Result<()> {
    let cdir = config_dir();
    println!("{}", cdir.display());
    Ok(())
}
