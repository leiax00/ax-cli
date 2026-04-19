use anyhow::Result;
use crate::config::{CommandStore, Config};

pub fn execute(name: &str, config: &Config) -> Result<()> {
    let cmd_path = crate::expand(&config.ax.commands_file);
    let mut map = CommandStore::load(&cmd_path)?;

    if CommandStore::remove(&mut map, name) {
        CommandStore::save(&cmd_path, &map)?;
        println!("🗑️  已删除: {name}");
        crate::commands::sync::execute(config)?;
    } else {
        println!("❌ 未找到: {name}");
    }

    Ok(())
}
