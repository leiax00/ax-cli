use crate::config::{
    expand_home, load_all_ssh_hosts, save_ssh_hosts, Config, SshAuth, SshHostEntry, SshHostMap,
};
use anyhow::{anyhow, bail, Result};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn add(
    name: &str,
    host: &str,
    user: &str,
    port: Option<u16>,
    auth: &str,
    password: Option<&str>,
    key: Option<&str>,
    desc: Option<&str>,
    config: &Config,
) -> Result<()> {
    let auth = parse_auth(auth)?;
    validate_auth(&auth, password)?;

    let mut map = load_all_ssh_hosts(config)?;
    map.insert(
        name.to_string(),
        SshHostEntry {
            host: host.to_string(),
            user: user.to_string(),
            port: port.unwrap_or(22),
            auth,
            password: password.map(ToOwned::to_owned).unwrap_or_default(),
            key: key.map(ToOwned::to_owned).unwrap_or_default(),
            desc: desc.map(ToOwned::to_owned).unwrap_or_default(),
        },
    );

    save_ssh_hosts(&map)?;
    println!("✅ 已保存 SSH 连接: {name}");
    Ok(())
}

pub fn list(config: &Config) -> Result<()> {
    let map = load_all_ssh_hosts(config)?;

    if map.is_empty() {
        println!("📋 暂无 SSH 连接");
        return Ok(());
    }

    println!("📋 SSH 连接列表：");
    println!("─────────────────────────────────────────────────────────");
    for (name, entry) in &map {
        let detail = if entry.desc.is_empty() {
            String::new()
        } else {
            format!("  # {}", entry.desc)
        };
        println!(
            "  {:<16} {}@{}:{} [{}]{}",
            name,
            entry.user,
            entry.host,
            entry.port,
            entry.auth.as_str(),
            detail
        );
    }
    Ok(())
}

pub fn rm(name: &str, config: &Config) -> Result<()> {
    let mut map = load_all_ssh_hosts(config)?;
    if map.remove(name).is_none() {
        bail!("❌ 未找到 SSH 连接: {name}");
    }

    save_ssh_hosts(&map)?;
    println!("🗑️  已删除 SSH 连接: {name}");
    Ok(())
}

pub fn connect(name: &str, config: &Config) -> Result<()> {
    let map = load_all_ssh_hosts(config)?;
    let entry = map
        .get(name)
        .ok_or_else(|| anyhow!("❌ 未找到 SSH 连接: {name}"))?;

    let target = format!("{}@{}", entry.user, entry.host);
    let port = entry.port.to_string();

    match entry.auth {
        SshAuth::Key => {
            let mut cmd = Command::new("ssh");
            cmd.arg("-p").arg(&port);
            if !entry.key.trim().is_empty() {
                cmd.arg("-i").arg(expand_home(&entry.key));
            }
            cmd.arg(&target);
            run_command(cmd)
        }
        SshAuth::Password => {
            if has_sshpass() {
                let mut cmd = Command::new("sshpass");
                cmd.arg("-e")
                    .env("SSHPASS", &entry.password)
                    .arg("ssh")
                    .arg("-p")
                    .arg(&port)
                    .arg(&target);
                run_command(cmd)
            } else {
                println!("⚠️  未检测到 sshpass，将切换为手动 SSH");
                println!("🔑 密码: {}", entry.password);
                let mut cmd = Command::new("ssh");
                cmd.arg("-p").arg(&port).arg(&target);
                run_command(cmd)
            }
        }
    }
}

pub fn select_and_connect(config: &Config) -> Result<()> {
    let map = load_all_ssh_hosts(config)?;

    if map.is_empty() {
        println!("📋 暂无 SSH 连接");
        return Ok(());
    }

    if has_fzf() {
        match select_with_fzf(&map)? {
            Some(name) => connect(&name, config),
            None => Ok(()),
        }
    } else {
        select_with_numbered(&map, config)
    }
}

fn select_with_fzf(map: &SshHostMap) -> Result<Option<String>> {
    let lines: Vec<String> = map
        .iter()
        .map(|(name, e)| {
            format!(
                "{}\t{}@{}:{}\t{}\t{}",
                name,
                e.user,
                e.host,
                e.port,
                e.auth.as_str(),
                if e.desc.is_empty() { String::new() } else { e.desc.clone() }
            )
        })
        .collect();

    let input = lines.join("\n");

    let mut child = Command::new("fzf")
        .args([
            "--prompt=SSH> ",
            "--header=选择 SSH 连接 (ESC 取消)",
            "--tiebreak=index",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input.as_bytes())?;
    }

    let output = child.wait_with_output()?;

    if !output.status.success() {
        return Ok(None);
    }

    let selected = String::from_utf8_lossy(&output.stdout);
    let name = selected.trim().split('\t').next().unwrap_or("").trim();

    if name.is_empty() {
        Ok(None)
    } else {
        Ok(Some(name.to_string()))
    }
}

fn select_with_numbered(map: &SshHostMap, config: &Config) -> Result<()> {
    println!("📋 SSH 连接列表：");
    println!("─────────────────────────────────────────────────────────");

    let entries: Vec<_> = map.iter().collect();

    for (i, (name, entry)) in entries.iter().enumerate() {
        let detail = if entry.desc.is_empty() {
            String::new()
        } else {
            format!("  # {}", entry.desc)
        };
        println!(
            "  {:3}) {:<16} {}@{}:{} [{}]{}",
            i + 1,
            name,
            entry.user,
            entry.host,
            entry.port,
            entry.auth.as_str(),
            detail
        );
    }

    println!("");
    print!("输入编号连接 (0 取消): ");
    std::io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim();

    if input == "0" || input.is_empty() {
        return Ok(());
    }

    if let Ok(idx) = input.parse::<usize>() {
        if idx > 0 && idx <= entries.len() {
            let (name, _) = entries[idx - 1];
            return connect(name, config);
        }
    }

    if map.contains_key(input) {
        return connect(input, config);
    }

    bail!("❌ 无效选择: {input}");
}

pub fn setup_key(
    name: &str,
    host: &str,
    user: &str,
    port: Option<u16>,
    password: Option<&str>,
    key: Option<&str>,
    desc: Option<&str>,
    config: &Config,
) -> Result<()> {
    let port = port.unwrap_or(22);
    let key_path = resolve_key_path(key);
    ensure_keypair(&key_path)?;

    let pubkey_path = public_key_path(&key_path)?;
    if has_ssh_copy_id() {
        copy_key_with_ssh_copy_id(user, host, port, password, &pubkey_path)?;
    } else {
        copy_key_with_fallback(user, host, port, password, &pubkey_path)?;
    }

    let key_string = key_path.to_string_lossy().to_string();
    add(
        name,
        host,
        user,
        Some(port),
        "key",
        None,
        Some(&key_string),
        desc,
        config,
    )?;
    println!("🔐 已配置 SSH key 免密登录");
    Ok(())
}

fn run_command(mut cmd: Command) -> Result<()> {
    let status = cmd
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;
    if status.success() {
        Ok(())
    } else {
        bail!("SSH 命令退出失败: {status}");
    }
}

fn has_sshpass() -> bool {
    which::which("sshpass").is_ok()
}

fn has_ssh_copy_id() -> bool {
    which::which("ssh-copy-id").is_ok()
}

fn has_fzf() -> bool {
    which::which("fzf").is_ok()
}

fn resolve_key_path(key: Option<&str>) -> PathBuf {
    match key {
        Some(path) => expand_home(path),
        None => expand_home("~/.ssh/id_ed25519"),
    }
}

fn ensure_keypair(key_path: &Path) -> Result<()> {
    let pubkey_path = public_key_path(key_path)?;
    if key_path.exists() && pubkey_path.exists() {
        return Ok(());
    }

    if let Some(parent) = key_path.parent() {
        fs::create_dir_all(parent)?;
    }

    println!("🔑 未检测到 SSH key，正在生成: {}", key_path.display());
    let status = Command::new("ssh-keygen")
        .args(["-t", "ed25519", "-f"])
        .arg(key_path)
        .args(["-N", ""])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;

    if status.success() {
        Ok(())
    } else {
        bail!("ssh-keygen 执行失败: {status}");
    }
}

fn public_key_path(key_path: &Path) -> Result<PathBuf> {
    let file_name = key_path
        .file_name()
        .ok_or_else(|| anyhow!("无效私钥路径: {}", key_path.display()))?;
    let pub_name = format!("{}.pub", file_name.to_string_lossy());
    Ok(key_path.with_file_name(pub_name))
}

fn copy_key_with_ssh_copy_id(
    user: &str,
    host: &str,
    port: u16,
    password: Option<&str>,
    pubkey_path: &Path,
) -> Result<()> {
    println!("📤 正在分发 SSH 公钥...");
    let target = format!("{user}@{host}");
    let status = match password {
        Some(password) => {
            if !has_sshpass() {
                bail!("提供了密码，但当前环境未安装 sshpass");
            }
            Command::new("sshpass")
                .arg("-e")
                .env("SSHPASS", password)
                .arg("ssh-copy-id")
                .arg("-i")
                .arg(pubkey_path)
                .args(["-p", &port.to_string()])
                .arg(&target)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()?
        }
        None => Command::new("ssh-copy-id")
            .arg("-i")
            .arg(pubkey_path)
            .args(["-p", &port.to_string()])
            .arg(&target)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?,
    };

    if status.success() {
        Ok(())
    } else {
        bail!("ssh-copy-id 执行失败: {status}");
    }
}

fn copy_key_with_fallback(
    user: &str,
    host: &str,
    port: u16,
    password: Option<&str>,
    pubkey_path: &Path,
) -> Result<()> {
    let pubkey = fs::read_to_string(pubkey_path)?.trim().to_string();
    if pubkey.is_empty() {
        bail!("公钥文件为空: {}", pubkey_path.display());
    }

    println!("📤 未检测到 ssh-copy-id，改用 authorized_keys 写入流程...");
    let target = format!("{user}@{host}");
    let escaped = pubkey.replace('\'', "'\\''");
    let remote_cmd = format!(
        "mkdir -p ~/.ssh && chmod 700 ~/.ssh && touch ~/.ssh/authorized_keys && chmod 600 ~/.ssh/authorized_keys && grep -qxF '{escaped}' ~/.ssh/authorized_keys || printf '%s\\n' '{escaped}' >> ~/.ssh/authorized_keys"
    );

    let status = match password {
        Some(password) => {
            if !has_sshpass() {
                bail!("提供了密码，但当前环境未安装 sshpass");
            }
            Command::new("sshpass")
                .arg("-e")
                .env("SSHPASS", password)
                .arg("ssh")
                .args(["-p", &port.to_string()])
                .arg(&target)
                .arg(&remote_cmd)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()?
        }
        None => Command::new("ssh")
            .args(["-p", &port.to_string()])
            .arg(&target)
            .arg(&remote_cmd)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?,
    };

    if status.success() {
        Ok(())
    } else {
        bail!("authorized_keys 写入失败: {status}");
    }
}

fn parse_auth(auth: &str) -> Result<SshAuth> {
    match auth.to_ascii_lowercase().as_str() {
        "key" => Ok(SshAuth::Key),
        "password" => Ok(SshAuth::Password),
        _ => bail!("认证方式仅支持 key 或 password"),
    }
}

fn validate_auth(auth: &SshAuth, password: Option<&str>) -> Result<()> {
    match auth {
        SshAuth::Key => Ok(()),
        SshAuth::Password => {
            if password.unwrap_or("").trim().is_empty() {
                bail!("auth=password 时必须提供 --password");
            }
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn parses_key_auth() {
        assert!(matches!(parse_auth("key").unwrap(), SshAuth::Key));
    }

    #[test]
    fn rejects_unknown_auth() {
        assert!(parse_auth("token").is_err());
    }

    #[test]
    fn requires_password_for_password_auth() {
        assert!(validate_auth(&SshAuth::Password, None).is_err());
    }

    #[test]
    fn defaults_key_path_to_ed25519() {
        assert!(resolve_key_path(None).ends_with(".ssh/id_ed25519"));
    }

    #[test]
    fn public_key_path_appends_pub_suffix() {
        let path = PathBuf::from("/tmp/id_ed25519");
        assert_eq!(
            public_key_path(&path).unwrap(),
            PathBuf::from("/tmp/id_ed25519.pub")
        );
    }
}
