use anyhow::Result;
use crate::config::{CommandStore, Config};

pub fn execute(name: &str, cmd: &str, desc: &str, config: &Config) -> Result<()> {
    let cmd_path = crate::expand(&config.ax.commands_file);
    let mut map = CommandStore::load(&cmd_path)?;

    if CommandStore::add(&mut map, name, cmd, desc) {
        CommandStore::save(&cmd_path, &map)?;
        println!("✅ 已添加: {name} - {desc}");
        crate::commands::sync::execute(config)?;
    } else {
        println!("⚠️  命令 '{name}' 已存在，请先删除或使用 edit");
    }

    Ok(())
}
