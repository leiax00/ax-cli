use anyhow::Result;

pub fn install_fzf() -> Result<()> {
    println!("🔍 检查 fzf...");
    if which::which("fzf").is_ok() {
        println!("  ⏭️  fzf 已存在");
    } else {
        let home = crate::expand("~");
        let fzf_dir = home.join(".fzf");
        let _ = std::process::Command::new("git")
            .args(["clone", "--depth", "1", "https://github.com/junegunn/fzf.git", fzf_dir.to_str().unwrap()])
            .output();
        let _ = std::process::Command::new("sh")
            .arg(fzf_dir.join("install"))
            .args(["--key-bindings", "--completion", "--no-update-rc"])
            .output();
        println!("  ✅ fzf 安装完成");
    }
    Ok(())
}

pub fn install_starship() -> Result<()> {
    println!("🚀 检查 starship...");
    if which::which("starship").is_ok() {
        println!("  ⏭️  starship 已存在");
    } else {
        let _ = std::process::Command::new("sh")
            .args(["-c", "curl -sS https://starship.rs/install.sh | sh -s -- -y"])
            .output();
        println!("  ✅ starship 安装完成");
    }
    Ok(())
}

pub fn check_font() -> Result<()> {
    println!("🔤 检查 Nerd Font...");
    let output = std::process::Command::new("fc-list")
        .output()?;
    let fc_output = String::from_utf8_lossy(&output.stdout);

    if fc_output.contains("JetBrains Mono") {
        println!("  ⏭️  字体已存在");
    } else {
        let font_dir = crate::expand("~/.local/share/fonts");
        std::fs::create_dir_all(&font_dir)?;
        let tmp = std::env::temp_dir().join("jetbrains-mono.zip");
        let _ = std::process::Command::new("curl")
            .args(["-fLo", tmp.to_str().unwrap(), "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/JetBrainsMono.zip"])
            .output();
        let _ = std::process::Command::new("unzip")
            .args(["-qo", tmp.to_str().unwrap(), "-d", font_dir.to_str().unwrap()])
            .output();
        let _ = std::fs::remove_file(&tmp);
        let _ = std::process::Command::new("fc-cache")
            .args(["-fv"])
            .output();
        println!("  ✅ JetBrains Mono Nerd Font 安装完成");
    }
    Ok(())
}
