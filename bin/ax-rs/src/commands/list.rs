use anyhow::Result;
use crate::config::{CommandStore, Config};

pub fn execute(config: &Config) -> Result<()> {
    let cmd_path = crate::expand(&config.ax.commands_file);
    let map = CommandStore::load(&cmd_path)?;

    if map.is_empty() {
        println!("📋 暂无自定义命令");
        println!("   使用 ax add <名称> <命令> [描述] 添加");
        return Ok(());
    }

    println!("📋 自定义命令列表：");
    println!("──────────────────────────────────────────");

    // 计算列宽
    let max_name = map.keys().map(|k| k.len()).max().unwrap_or(0);
    let max_desc = map.values().map(|v| v.desc.len()).max().unwrap_or(0);
    let name_w = max_name.max(4);
    let desc_w = max_desc.max(4);

    for (name, entry) in &map {
        println!("  {:width$}  {:desc_width$}  → {}", name, entry.desc, entry.cmd,
            width = name_w, desc_width = desc_w);
    }

    println!("──────────────────────────────────────────");
    Ok(())
}
