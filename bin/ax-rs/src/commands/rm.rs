use anyhow::Result;
use crate::config::{Config, load_all_commands, save_commands};

pub fn execute(name: &str, config: &Config) -> Result<()> {
    let mut map = load_all_commands(config)?;

    if map.remove(name).is_some() {
        save_commands(&map)?;
        println!("🗑️  已删除: {name}");
        crate::commands::sync::execute(config)?;
    } else {
        println!("❌ 未找到: {name}");
    }

    Ok(())
}
