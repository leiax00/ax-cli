use std::io::Write;
use anyhow::{bail, Result};
use crate::config::{CommandStore, Config};

pub fn execute(name: Option<&str>, config: &Config) -> Result<()> {
    let cmd_path = crate::expand(&config.ax.commands_file);
    let map = CommandStore::load(&cmd_path)?;

    match name {
        Some(n) => {
            if let Some(entry) = CommandStore::get(&map, n) {
                println!("▶ {}", entry.cmd);
                std::process::Command::new("sh")
                    .arg("-c")
                    .arg(&entry.cmd)
                    .status()?;
            } else {
                bail!("❌ 未找到: {n}");
            }
        }
        None => {
            // 无参数时列出所有命令供选择
            if map.is_empty() {
                println!("📋 暂无自定义命令");
                return Ok(());
            }

            println!("📋 可用命令：");
            let mut entries: Vec<_> = map.iter().collect();
            entries.sort_by_key(|(k, _)| *k);

            for (i, (name, entry)) in entries.iter().enumerate() {
                let desc = if entry.desc.is_empty() { "无描述" } else { &entry.desc };
                println!("  {:3}) {:<20} {}", i + 1, name, desc);
            }

            println!("");
            print!("输入编号执行 (0 取消): ");
            std::io::stdout().flush()?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let input = input.trim();

            if input == "0" || input.is_empty() {
                return Ok(());
            }

            if let Ok(idx) = input.parse::<usize>() {
                if idx > 0 && idx <= entries.len() {
                    let (_name, entry) = entries[idx - 1];
                    println!("▶ {}", entry.cmd);
                    std::process::Command::new("sh")
                        .arg("-c")
                        .arg(&entry.cmd)
                        .status()?;
                } else {
                    // 尝试按名称匹配
                    if let Some(entry) = CommandStore::get(&map, input) {
                        println!("▶ {}", entry.cmd);
                        std::process::Command::new("sh")
                            .arg("-c")
                            .arg(&entry.cmd)
                            .status()?;
                    } else {
                        bail!("❌ 无效选择: {input}");
                    }
                }
            } else {
                bail!("❌ 无效输入: {input}");
            }
        }
    }

    Ok(())
}
