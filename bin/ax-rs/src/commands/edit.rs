use anyhow::Result;
use std::io::{self, Write};

use crate::config::{CommandStore, Config};

pub fn execute(name: &str, config: &Config) -> Result<()> {
    let cmd_path = crate::expand(&config.ax.commands_file);
    let mut map = CommandStore::load(&cmd_path)?;

    if let Some(entry) = CommandStore::get(&map, name) {
        println!("当前命令: {}", entry.cmd);
        println!("当前描述: {}", entry.desc);

        print!("新命令 (直接回车保持不变): ");
        io::stdout().flush()?;
        let mut new_cmd = String::new();
        io::stdin().read_line(&mut new_cmd)?;
        let new_cmd = new_cmd.trim();
        let new_cmd = if new_cmd.is_empty() { &entry.cmd } else { new_cmd };

        print!("新描述 (直接回车保持不变): ");
        io::stdout().flush()?;
        let mut new_desc = String::new();
        io::stdin().read_line(&mut new_desc)?;
        let new_desc = new_desc.trim();
        let new_desc = if new_desc.is_empty() { &entry.desc } else { new_desc };

        map.insert(name.into(), crate::config::CommandEntry {
            cmd: new_cmd.into(),
            desc: new_desc.into(),
        });
        CommandStore::save(&cmd_path, &map)?;
        println!("✅ 已更新: {name}");
        crate::commands::sync::execute(config)?;
    } else {
        println!("❌ 未找到: {name}");
    }

    Ok(())
}
