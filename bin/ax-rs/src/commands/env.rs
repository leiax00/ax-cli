use anyhow::Result;
use crate::config::{Config, EnvEntry, load_all_env, save_env};

pub fn add(name: &str, value: &str, desc: &str, tags: &[String], config: &Config) -> Result<()> {
    let mut map = load_all_env(config)?;
    map.insert(name.to_string(), EnvEntry {
        value: value.to_string(),
        desc: desc.to_string(),
        tags: tags.to_vec(),
        paused: false,
    });
    save_env(&map)?;
    println!("✅ 已添加: {name}={value}");
    if !tags.is_empty() {
        println!("   标签: {}", tags.join(", "));
    }
    crate::commands::config::push(config)?;
    Ok(())
}

pub fn edit(name: &str, value: Option<&str>, desc: Option<&str>, tags: Option<&[String]>, config: &Config) -> Result<()> {
    let mut map = load_all_env(config)?;
    let entry = map.get_mut(name).ok_or_else(|| anyhow::anyhow!("❌ 未找到: {name}"))?;

    if let Some(v) = value { entry.value = v.to_string(); }
    if let Some(d) = desc { entry.desc = d.to_string(); }
    if let Some(t) = tags { entry.tags = t.to_vec(); }

    save_env(&map)?;
    println!("✅ 已修改: {name}");
    crate::commands::config::push(config)?;
    Ok(())
}

pub fn rm(names: &[String], config: &Config) -> Result<()> {
    let mut map = load_all_env(config)?;
    let mut count = 0u32;
    for name in names {
        if map.remove(name).is_some() {
            println!("🗑️  已删除: {name}");
            count += 1;
        } else {
            println!("⚠️  未找到: {name}");
        }
    }
    if count > 0 {
        save_env(&map)?;
        crate::commands::config::push(config)?;
    }
    Ok(())
}

pub fn show(name: Option<&str>, tag: Option<&str>, all: bool, config: &Config) -> Result<()> {
    let map = load_all_env(config)?;

    let filtered: Vec<_> = match (name, tag, all) {
        (Some(n), _, _) => map.iter().filter(|(k, _)| *k == n).collect(),
        (_, Some(t), _) => map.iter().filter(|(_, v)| v.tags.iter().any(|tg| tg == t)).collect(),
        (_, _, true) => map.iter().collect(),
        _ => map.iter().collect(),
    };

    if filtered.is_empty() {
        println!("📋 暂无环境变量");
        return Ok(());
    }

    // 计算列宽
    let max_name = filtered.iter().map(|(k, _)| k.len()).max().unwrap_or(0).max(6);
    let max_val = filtered.iter().map(|(_, v)| v.value.len()).max().unwrap_or(0).min(50);

    println!("📋 环境变量列表：");
    println!("─────────────────────────────────────────────────────────");

    for (name, entry) in &filtered {
        let status = if entry.paused { "⏸️ " } else { "✅ " };
        let tags = if entry.tags.is_empty() { String::new() } else { format!(" [{}]", entry.tags.join(",")) };
        let desc = if entry.desc.is_empty() { String::new() } else { format!("  # {}", entry.desc) };
        let val_display = if entry.value.len() > 50 {
            format!("{}...", &entry.value[..47])
        } else {
            entry.value.clone()
        };
        println!("  {}{:<name_w$} = {:<val_w$}{}{}", status, name, val_display, tags, desc,
            name_w = max_name, val_w = max_val);
    }

    println!("─────────────────────────────────────────────────────────");
    println!("  共 {} 个变量", filtered.len());
    Ok(())
}

pub fn pause(names: &[String], tag: Option<&str>, all: bool, config: &Config) -> Result<()> {
    let mut map = load_all_env(config)?;
    let mut count = 0u32;

    for (name, entry) in map.iter_mut() {
        if all {
            if !entry.paused { entry.paused = true; count += 1; }
        } else if let Some(t) = tag {
            if entry.tags.iter().any(|tg| tg == t) && !entry.paused { entry.paused = true; count += 1; }
        } else if names.iter().any(|n| n == name) {
            entry.paused = true; count += 1;
        }
    }

    if count == 0 {
        println!("⏭️  没有变量被暂停");
        return Ok(());
    }

    save_env(&map)?;
    println!("⏸️  已暂停 {count} 个变量");
    crate::commands::config::push(config)?;
    Ok(())
}

pub fn resume(names: &[String], tag: Option<&str>, all: bool, config: &Config) -> Result<()> {
    let mut map = load_all_env(config)?;
    let mut count = 0u32;

    for (name, entry) in map.iter_mut() {
        if all {
            if entry.paused { entry.paused = false; count += 1; }
        } else if let Some(t) = tag {
            if entry.tags.iter().any(|tg| tg == t) && entry.paused { entry.paused = false; count += 1; }
        } else if names.iter().any(|n| n == name) {
            entry.paused = false; count += 1;
        }
    }

    if count == 0 {
        println!("⏭️  没有变量被恢复");
        return Ok(());
    }

    save_env(&map)?;
    println!("▶️  已恢复 {count} 个变量");
    crate::commands::config::push(config)?;
    Ok(())
}

/// 输出 shell export 命令（eval $(ax env load) 用）
pub fn load(config: &Config) -> Result<()> {
    let map = load_all_env(config)?;
    for (name, entry) in &map {
        if !entry.paused {
            println!("export {}={}", name, shell_escape(&entry.value));
        }
    }
    Ok(())
}

/// 显示所有标签
pub fn tags(config: &Config) -> Result<()> {
    let map = load_all_env(config)?;
    let mut all_tags: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for entry in map.values() {
        for tag in &entry.tags {
            all_tags.insert(tag.clone());
        }
    }

    if all_tags.is_empty() {
        println!("📋 暂无标签");
        return Ok(());
    }

    println!("🏷️  标签列表：");
    for tag in &all_tags {
        let count = map.values().filter(|e| e.tags.iter().any(|t| t == tag)).count();
        let paused_count = map.values().filter(|e| e.tags.iter().any(|t| t == tag) && e.paused).count();
        let status = if paused_count > 0 { format!(" ({} 个暂停)", paused_count) } else { String::new() };
        println!("  {:<20} {} 个变量{}", tag, count, status);
    }
    Ok(())
}

fn shell_escape(s: &str) -> String {
    if s.contains(' ') || s.contains('\'') || s.contains('"') || s.contains('$') {
        format!("'{}'", s.replace('\'', "'\\''"))
    } else {
        s.to_string()
    }
}
