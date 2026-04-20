use crate::config::Config;
use crate::detect;
use anyhow::Result;

pub fn check_and_install(config: &Config, include_extras: bool) -> Result<()> {
    println!("📦 检查系统包...");

    let packages_dir = crate::expand(&config.packages.dir);
    let pkg_file = crate::expand(&detect::packages_file());

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
    let wanted = parse_packages(&content, include_extras);
    let mut new_pkgs = Vec::new();

    for pkg in wanted {
        if !detect::is_package_installed(&pkg) {
            new_pkgs.push(pkg);
        }
    }

    if new_pkgs.is_empty() {
        println!("  ⏭️  系统包齐全");
    } else {
        let scope = if include_extras {
            "core + extras"
        } else {
            "core"
        };
        println!("  📥 新增包 ({scope}): {}", new_pkgs.join(", "));
        install_packages(&new_pkgs)?;
        println!("  ✅ 安装完成");
    }

    Ok(())
}

fn parse_packages(content: &str, include_extras: bool) -> Vec<String> {
    let mut packages = Vec::new();
    let mut section = PackageSection::Legacy;

    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        section = match line {
            "[core]" => PackageSection::Core,
            "[extras]" => PackageSection::Extras,
            _ => section,
        };

        if matches!(line, "[core]" | "[extras]") {
            continue;
        }

        if section == PackageSection::Extras && !include_extras {
            continue;
        }

        packages.push(line.to_string());
    }

    packages
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PackageSection {
    Legacy,
    Core,
    Extras,
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

#[cfg(test)]
mod tests {
    use super::parse_packages;

    #[test]
    fn parses_core_only_when_extras_disabled() {
        let content = "[core]\n# package manager\n git\ncurl\n[extras]\nfzf\nbat\n";
        let pkgs = parse_packages(content, false);
        assert_eq!(pkgs, vec!["git", "curl"]);
    }

    #[test]
    fn parses_core_and_extras_when_enabled() {
        let content = "[core]\ngit\ncurl\n[extras]\nfzf\nbat\n";
        let pkgs = parse_packages(content, true);
        assert_eq!(pkgs, vec!["git", "curl", "fzf", "bat"]);
    }
}
