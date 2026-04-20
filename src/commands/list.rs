use crate::config::{load_all_commands, Config};
use anyhow::Result;

pub fn execute(config: &Config) -> Result<()> {
    let map = load_all_commands(config)?;

    if map.is_empty() {
        println!("📋 暂无自定义命令");
        println!("   使用 ax add <名称> <命令> [描述] 添加");
        return Ok(());
    }

    println!("📋 自定义命令列表：");
    println!("──────────────────────────────────────────");

    let max_name = map.keys().map(|k| k.len()).max().unwrap_or(0);
    let max_desc = map.values().map(|v| v.desc.len()).max().unwrap_or(0);
    let name_w = max_name.max(4);
    let desc_w = max_desc.max(4);

    for (name, entry) in &map {
        println!(
            "  {:width$}  {:desc_width$}  → {}",
            name,
            entry.desc,
            entry.cmd,
            width = name_w,
            desc_width = desc_w
        );
    }

    println!("──────────────────────────────────────────");
    Ok(())
}

/// 只输出命令名，供补全脚本调用
pub fn execute_quiet(config: &Config) -> Result<()> {
    let map = load_all_commands(config)?;
    for name in map.keys() {
        println!("{name}");
    }
    Ok(())
}
