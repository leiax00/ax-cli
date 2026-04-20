use crate::config::{load_all_env, save_env, Config, EnvEntry};
use anyhow::Result;

pub fn add(name: &str, value: &str, desc: &str, tags: &[String], config: &Config) -> Result<()> {
    let mut map = load_all_env(config)?;
    map.insert(
        name.to_string(),
        EnvEntry {
            value: value.to_string(),
            desc: desc.to_string(),
            tags: tags.to_vec(),
            paused: false,
        },
    );
    save_env(&map)?;
    println!("✅ 已添加: {name}={value}");
    if !tags.is_empty() {
        println!("   标签: {}", tags.join(", "));
    }
    crate::commands::config::push(config)?;
    Ok(())
}

pub fn edit(
    name: &str,
    value: Option<&str>,
    desc: Option<&str>,
    tags: Option<&[String]>,
    config: &Config,
) -> Result<()> {
    let mut map = load_all_env(config)?;
    let entry = map
        .get_mut(name)
        .ok_or_else(|| anyhow::anyhow!("❌ 未找到: {name}"))?;

    if let Some(v) = value {
        entry.value = v.to_string();
    }
    if let Some(d) = desc {
        entry.desc = d.to_string();
    }
    if let Some(t) = tags {
        entry.tags = t.to_vec();
    }

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
        (_, Some(t), _) => map
            .iter()
            .filter(|(_, v)| v.tags.iter().any(|tg| tg == t))
            .collect(),
        (_, _, true) => map.iter().collect(),
        _ => map.iter().collect(),
    };

    if filtered.is_empty() {
        println!("📋 暂无环境变量");
        return Ok(());
    }

    // 计算列宽
    let max_name = filtered
        .iter()
        .map(|(k, _)| k.len())
        .max()
        .unwrap_or(0)
        .max(6);
    let max_val = filtered
        .iter()
        .map(|(_, v)| v.value.len())
        .max()
        .unwrap_or(0)
        .min(50);

    println!("📋 环境变量列表：");
    println!("─────────────────────────────────────────────────────────");

    for (name, entry) in &filtered {
        let status = if entry.paused { "⏸️ " } else { "✅ " };
        let tags = if entry.tags.is_empty() {
            String::new()
        } else {
            format!(" [{}]", entry.tags.join(","))
        };
        let desc = if entry.desc.is_empty() {
            String::new()
        } else {
            format!("  # {}", entry.desc)
        };
        let val_display = if entry.value.len() > 50 {
            format!("{}...", &entry.value[..47])
        } else {
            entry.value.clone()
        };
        println!(
            "  {}{:<name_w$} = {:<val_w$}{}{}",
            status,
            name,
            val_display,
            tags,
            desc,
            name_w = max_name,
            val_w = max_val
        );
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
            if !entry.paused {
                entry.paused = true;
                count += 1;
            }
        } else if let Some(t) = tag {
            if entry.tags.iter().any(|tg| tg == t) && !entry.paused {
                entry.paused = true;
                count += 1;
            }
        } else if names.iter().any(|n| n == name) {
            entry.paused = true;
            count += 1;
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
            if entry.paused {
                entry.paused = false;
                count += 1;
            }
        } else if let Some(t) = tag {
            if entry.tags.iter().any(|tg| tg == t) && entry.paused {
                entry.paused = false;
                count += 1;
            }
        } else if names.iter().any(|n| n == name) {
            entry.paused = false;
            count += 1;
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

/// 输出 shell export 命令
/// 自动检测 shell 类型，输出对应格式
/// bash/zsh: eval $(ax env load)
/// powershell: Invoke-Expression "$(ax env load)"
/// cmd: for /f "delims=" %i in ('ax env load') do %i
pub fn load(config: &Config) -> Result<()> {
    let map = load_all_env(config)?;
    let shell_type = detect_shell_type();

    for (name, entry) in &map {
        if entry.paused {
            continue;
        }
        match shell_type {
            ShellType::PowerShell => {
                println!("$env:{name} = \"{}\"", ps_escape(&entry.value));
            }
            ShellType::Cmd => {
                println!("set {}={}", name, entry.value);
            }
            ShellType::Bash => {
                println!("export {}={}", name, bash_escape(&entry.value));
            }
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
        let count = map
            .values()
            .filter(|e| e.tags.iter().any(|t| t == tag))
            .count();
        let paused_count = map
            .values()
            .filter(|e| e.tags.iter().any(|t| t == tag) && e.paused)
            .count();
        let status = if paused_count > 0 {
            format!(" ({} 个暂停)", paused_count)
        } else {
            String::new()
        };
        println!("  {:<20} {} 个变量{}", tag, count, status);
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ShellType {
    Bash,
    PowerShell,
    Cmd,
}

/// 检测当前 shell 类型
fn detect_shell_type() -> ShellType {
    // 检查 AX_SHELL 环境变量（手动指定）
    if let Ok(s) = std::env::var("AX_SHELL") {
        return match s.to_lowercase().as_str() {
            "powershell" | "pwsh" => ShellType::PowerShell,
            "cmd" | "cmd.exe" => ShellType::Cmd,
            _ => ShellType::Bash,
        };
    }

    // 通过父进程检测
    let parent = std::env::var("SHELL").unwrap_or_default();
    if parent.contains("pwsh") || parent.contains("powershell") {
        return ShellType::PowerShell;
    }

    #[cfg(target_os = "windows")]
    {
        // Windows: 检查 PSModulePath 或当前进程名
        if std::env::var("PSModulePath").is_ok() {
            // 进一步区分 pwsh 和 powershell
            let exe = std::env::current_exe()
                .ok()
                .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
                .unwrap_or_default();
            // 检查父进程是否是 cmd
            let pp = std::env::var("PROMPT").unwrap_or_default();
            if !pp.is_empty() {
                return ShellType::Cmd;
            }
            return ShellType::PowerShell;
        }
        return ShellType::Cmd;
    }

    ShellType::Bash
}

fn bash_escape(s: &str) -> String {
    let needs = s.contains(' ')
        || s.contains('\x27')
        || s.contains('"')
        || s.contains('$')
        || s.contains('\\');
    if needs {
        let escaped = s.replace("\x27", "'\\x27''");
        format!("'{}'", escaped)
    } else {
        s.to_string()
    }
}

fn ps_escape(s: &str) -> String {
    s.replace('"', "`\"")
}
