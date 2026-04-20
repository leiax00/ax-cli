/// 系统检测模块

/// 获取 OS ID (ubuntu, fedora, arch, macos, windows)
pub fn os_id() -> String {
    #[cfg(target_os = "macos")]
    {
        return "macos".into();
    }
    #[cfg(target_os = "windows")]
    {
        return "windows".into();
    }
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
            for line in content.lines() {
                if let Some(id) = line.strip_prefix("ID=") {
                    return id.trim_matches('"').trim().to_string();
                }
            }
        }
    }
    "unknown".into()
}

/// 获取 OS 名称
pub fn os_name() -> String {
    #[cfg(target_os = "macos")]
    {
        return "macOS".into();
    }
    #[cfg(target_os = "windows")]
    {
        return "Windows".into();
    }
    #[cfg(target_os = "linux")]
    {
        if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
            for line in content.lines() {
                if let Some(name) = line.strip_prefix("NAME=") {
                    return name.trim_matches('"').trim().to_string();
                }
            }
        }
    }
    "Unknown".into()
}

/// 获取包管理器名称
pub fn pkg_manager() -> &'static str {
    match os_id().as_str() {
        "ubuntu" | "debian" | "linuxmint" | "pop" => "apt",
        "fedora" | "rhel" | "centos" | "rocky" | "alma" => "dnf",
        "arch" | "manjaro" | "endeavouros" => "pacman",
        "macos" => "brew",
        _ => "unknown",
    }
}

/// 获取包列表文件名
pub fn packages_file() -> String {
    match os_id().as_str() {
        "macos" => "packages/brew.txt".into(),
        "windows" => "packages/choco.txt".into(),
        id => format!("packages/{id}.txt"),
    }
}

/// 检查包是否已安装
pub fn is_package_installed(pkg: &str) -> bool {
    match pkg_manager() {
        "apt" => std::process::Command::new("dpkg")
            .args(["-s", pkg])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false),
        "dnf" => std::process::Command::new("rpm")
            .args(["-q", pkg])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false),
        "pacman" => std::process::Command::new("pacman")
            .args(["-Q", pkg])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false),
        "brew" => which::which(pkg).is_ok(),
        _ => false,
    }
}
