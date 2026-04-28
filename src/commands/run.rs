use crate::config::{command_with_forwarded_args, load_all_commands, Config};
use anyhow::{bail, Result};
use std::io::Write;

pub fn execute(name: Option<&str>, args: &[String], config: &Config) -> Result<()> {
    let map = load_all_commands(config)?;

    match name {
        Some(n) => {
            if let Some(entry) = map.get(n) {
                println!("▶ {}", entry.cmd);
                std::process::Command::new("sh")
                    .arg("-c")
                    .arg(command_with_forwarded_args(&entry.cmd))
                    .arg("ax-run")
                    .args(args)
                    .status()?;
            } else {
                bail!("❌ 未找到: {n}");
            }
        }
        None => {
            if map.is_empty() {
                println!("📋 暂无自定义命令");
                return Ok(());
            }

            println!("📋 可用命令：");
            let mut entries: Vec<_> = map.iter().collect();
            entries.sort_by_key(|(k, _)| *k);

            for (i, (name, entry)) in entries.iter().enumerate() {
                let desc = if entry.desc.is_empty() {
                    "无描述"
                } else {
                    entry.desc.as_str()
                };
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
                        .arg(command_with_forwarded_args(&entry.cmd))
                        .arg("ax-run")
                        .args(args)
                        .status()?;
                } else if let Some(entry) = map.get(input) {
                    println!("▶ {}", entry.cmd);
                    std::process::Command::new("sh")
                        .arg("-c")
                        .arg(command_with_forwarded_args(&entry.cmd))
                        .arg("ax-run")
                        .args(args)
                        .status()?;
                } else {
                    bail!("❌ 无效选择: {input}");
                }
            }
        }
    }

    Ok(())
}
