use crate::config::{generate_command_functions, load_all_commands, save_commands, Config};
use anyhow::Result;

pub fn execute(name: &str, config: &Config) -> Result<()> {
    let mut map = load_all_commands(config)?;
    let name = resolve_name(name, &map);

    if map.remove(&name).is_some() {
        save_commands(&map)?;
        generate_command_functions(config)?;
        println!("🗑️  已删除: {name}");
    } else {
        println!("❌ 未找到: {name}");
    }

    Ok(())
}

fn resolve_name(name: &str, map: &crate::config::CommandMap) -> String {
    if map.contains_key(name) {
        name.to_string()
    } else {
        let prefixed = format!("ax-{name}");
        if map.contains_key(&prefixed) {
            prefixed
        } else {
            name.to_string()
        }
    }
}
