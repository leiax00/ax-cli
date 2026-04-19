use anyhow::Result;
use crate::config::{Config, CommandStore, expand_home, config_dir, TEMPLATE_CONFIG_YAML, TEMPLATE_ZSHRC, TEMPLATE_WEZTERM};

pub fn execute(force: bool, _config: &Config) -> Result<()> {
    let cdir = config_dir();

    if cdir.exists() && !force {
        println!("⚠️  配置目录已存在: {}", cdir.display());
        println!("   使用 ax init --force 强制覆盖");
        println!("   已有配置不会被删除，只会覆盖同名文件");
    }

    println!("🚀 初始化 ax-cli...");
    println!("   配置目录: {}", cdir.display());

    // 1. 创建目录结构
    std::fs::create_dir_all(cdir.join("config.d"))?;
    std::fs::create_dir_all(cdir.join("bash"))?;
    std::fs::create_dir_all(cdir.join("wezterm"))?;
    std::fs::create_dir_all(cdir.join("packages"))?;
    std::fs::create_dir_all(cdir.join("git"))?;
    println!("   ✅ 目录结构创建完成");

    // 2. 写入默认 config.yaml
    let config_file = cdir.join("config.yaml");
    if !config_file.exists() || force {
        std::fs::write(&config_file, TEMPLATE_CONFIG_YAML)?;
        println!("   ✅ config.yaml");
    } else {
        println!("   ⏭️  config.yaml 已存在");
    }

    // 3. 写入 .zshrc 模板
    let zshrc_file = cdir.join("bash").join(".zshrc");
    if !zshrc_file.exists() || force {
        std::fs::write(&zshrc_file, TEMPLATE_ZSHRC)?;
        println!("   ✅ bash/.zshrc");
    } else {
        println!("   ⏭️  bash/.zshrc 已存在");
    }

    // 4. 写入 wezterm.lua 模板
    let wezterm_file = cdir.join("wezterm").join("wezterm.lua");
    if !wezterm_file.exists() || force {
        std::fs::write(&wezterm_file, TEMPLATE_WEZTERM)?;
        println!("   ✅ wezterm/wezterm.lua");
    } else {
        println!("   ⏭️  wezterm/wezterm.lua 已存在");
    }

    // 5. 写入默认包列表（根据当前系统）
    let pkg_file = cdir.join("packages").join(format!("{}.txt", crate::detect::os_id()));
    if !pkg_file.exists() {
        let default_pkgs = match crate::detect::os_id().as_str() {
            "ubuntu" | "debian" => "jq\nfzf\ngit\ncurl\nwget\ntree\nhtop\nripgrep\nfd-find\nbat\ntmux\nzsh\npython3-pip\nnodejs\nnpm\ndocker.io\ndocker-compose-v2\n",
            "fedora" | "rhel" | "centos" => "jq\nfzf\ngit\ncurl\nwget\ntree\nhtop\nripgrep\nfd-find\nbat\ntmux\nzsh\npython3-pip\nnodejs\nnpm\ndocker-ce\ndocker-compose-plugin\n",
            "arch" | "manjaro" => "jq\nfzf\ngit\ncurl\nwget\ntree\nhtop\nripgrep\nfd\nbat\ntmux\nzsh\npython-pip\nnodejs\nnpm\ndocker\ndocker-compose\n",
            _ => "jq\nfzf\ngit\ncurl\nwget\ntree\nhtop\nripgrep\nbat\ntmux\nzsh\n",
        };
        std::fs::write(&pkg_file, default_pkgs)?;
        println!("   ✅ packages/{}.txt", crate::detect::os_id());
    } else {
        println!("   ⏭️  packages/{}.txt 已存在", crate::detect::os_id());
    }

    // 6. 创建命令库
    let cmd_file = expand_home("~/ax-commands.json");
    if !cmd_file.exists() {
        CommandStore::save(&cmd_file, &CommandStore::load(&cmd_file)?)?;
        println!("   ✅ ax-commands.json");
    } else {
        println!("   ⏭️  ax-commands.json 已存在");
    }

    println!("");
    println!("✅ 初始化完成！");
    println!("");
    println!("下一步：");
    println!("  ax install    # 安装系统包、工具、部署配置");
    println!("  ax info       # 查看当前配置路径");
    println!("");
    println!("配置文件: {}", cdir.join("config.yaml").display());

    Ok(())
}
