use anyhow::Result;
use crate::config::Config;
use crate::detect;

pub fn check_and_install(config: &Config) -> Result<()> {
    println!("📦 检查系统包...");

    let packages_dir = crate::expand(&config.packages.dir);
    let pkg_file = crate::expand(&detect::packages_file());

    // 优先读 packages/{os}.txt，不存在则读 packages_dir 下的
    let file = if pkg_file.exists() {
        pkg_file
    } else {
        let fallback = packages_dir.join(format!("{}.txt", detect::os_id()));
        if fallback.exists() {
            fallback
        } else {
            println!("  ⚠️  未找到包列表文件");
            return Ok(());
        }
    };

    let content = std::fs::read_to_string(&file)?;
    let mut new_pkgs = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if !detect::is_package_installed(line) {
            new_pkgs.push(line.to_string());
        }
    }

    if new_pkgs.is_empty() {
        println!("  ⏭️  系统包齐全");
    } else {
        println!("  📥 新增包: {}", new_pkgs.join(", "));
        install_packages(&new_pkgs)?;
        println!("  ✅ 安装完成");
    }

    Ok(())
}

fn install_packages(pkgs: &[String]) -> Result<()> {
    match detect::pkg_manager() {
        "apt" => {
            std::process::Command::new("sudo")
                .args(["apt", "update", "-qq"])
                .status()?;
            std::process::Command::new("sudo")
                .args(["apt", "install", "-y", "-qq"])
                .args(pkgs)
                .status()?;
        }
        "dnf" => {
            std::process::Command::new("sudo")
                .args(["dnf", "install", "-y"])
                .args(pkgs)
                .status()?;
        }
        "pacman" => {
            std::process::Command::new("sudo")
                .args(["pacman", "-S", "--noconfirm", "--needed"])
                .args(pkgs)
                .status()?;
        }
        "brew" => {
            std::process::Command::new("brew")
                .args(["install"])
                .args(pkgs)
                .status()?;
        }
        mgr => println!("  ⚠️  不支持的包管理器: {mgr}"),
    }
    Ok(())
}
