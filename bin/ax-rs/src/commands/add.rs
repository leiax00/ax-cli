use anyhow::Result;
use crate::config::{Config, load_all_commands, save_commands};

pub fn execute(name: &str, cmd: &str, desc: &str, config: &Config) -> Result<()> {
    let mut map = load_all_commands(config)?;

    if map.contains_key(name) {
        println!("⚠️  命令 '{name}' 已存在，请先删除或使用 edit");
        return Ok(());
    }

    map.insert(name.into(), crate::config::CommandEntry {
        cmd: cmd.into(),
        desc: desc.into(),
    });
    save_commands(&map)?;
    println!("✅ 已添加: {name} - {desc}");
    crate::commands::config::push(config)?;

    Ok(())
}
