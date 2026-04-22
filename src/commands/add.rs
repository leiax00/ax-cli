use crate::config::{config_dir, generate_command_functions, load_all_commands, save_commands, Config};
use anyhow::{bail, Result};
use std::io::{self, Read};

pub fn execute(
    name: &str,
    cmd: Option<&str>,
    desc: Option<&str>,
    file: Option<&str>,
    raw: bool,
    config: &Config,
) -> Result<()> {
    let mut map = load_all_commands(config)?;
    let name = if raw { name.to_string() } else { ensure_prefix(name) };

    if map.contains_key(&name) {
        println!("⚠️  命令 '{name}' 已存在，请先删除或使用 edit");
        return Ok(());
    }

    let cmd = resolve_cmd(cmd, file)?;

    if cmd.trim().is_empty() {
        bail!("命令内容不能为空");
    }

    let desc = desc.unwrap_or("");

    map.insert(
        name.clone(),
        crate::config::CommandEntry {
            cmd: cmd.clone(),
            desc: desc.into(),
        },
    );
    save_commands(&map)?;
    generate_command_functions(config)?;
    println!("✅ 已添加: {name} - {desc}");
    println!("   运行 source {}/config.d/commands.sh 使命令生效", config_dir().display());

    Ok(())
}

fn resolve_cmd(cmd: Option<&str>, file: Option<&str>) -> Result<String> {
    match (cmd, file) {
        // -f 指定文件
        (None, Some(path)) => Ok(std::fs::read_to_string(path)?),
        // - 从 stdin 读取
        (Some("-"), None) => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            Ok(buf)
        }
        // 直接传入命令
        (Some(c), None) => Ok(c.to_string()),
        // 两者都指定，报错
        (Some(_), Some(_)) => bail!("不能同时指定命令内容和 -f 参数"),
        // 都不传，交互式逐行输入
        (None, None) => read_multiline(),
    }
}

fn read_multiline() -> Result<String> {
    println!("请输入命令内容（空行结束）：");
    let mut lines = Vec::new();
    let mut line = String::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line)?;
        if line.trim().is_empty() {
            break;
        }
        lines.push(line.trim_end().to_string());
    }
    Ok(lines.join("\n"))
}

fn ensure_prefix(name: &str) -> String {
    if name.starts_with("ax-") {
        name.to_string()
    } else {
        format!("ax-{name}")
    }
}
