use crate::config::Config;
use anyhow::Result;

pub fn execute(config: &Config) -> Result<()> {
    // 拉取配置
    crate::commands::config::pull(config)?;

    // 检查系统包
    println!("");
    crate::packages::check_and_install(config, false)?;

    // 更新 zsh 插件
    println!("");
    crate::shell::update_plugins(config)?;

    // 检查字体
    println!("");
    crate::tools::check_font()?;

    println!("");
    println!("✅ 更新完成！重启终端，或重新加载对应的 shell 入口文件使配置生效。");
    Ok(())
}
