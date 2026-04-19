use anyhow::Result;
use crate::config::Config;

pub fn execute(config: &Config) -> Result<()> {
    // 拉取配置
    crate::commands::config::pull(config)?;

    // 检查系统包
    println!("");
    crate::packages::check_and_install(config)?;

    // 更新 zsh 插件
    println!("");
    crate::shell::update_plugins(config)?;

    // 检查字体
    println!("");
    crate::tools::check_font()?;

    println!("");
    println!("✅ 更新完成！运行 exec zsh 或重启终端使配置生效。");
    Ok(())
}
