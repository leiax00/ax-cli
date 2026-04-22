use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// 全局配置结构
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default)]
pub struct Config {
    pub ax: AxConfig,
    pub proxy: ProxyConfig,
    pub shell: ShellConfig,
    pub packages: PackagesConfig,
    pub deploy: DeployConfig,
    /// 环境变量（可直接在 config.yaml 中定义）
    #[serde(default)]
    pub env: EnvMap,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AxConfig {
    pub auto_sync: bool,
    /// 命令列表（直接在 config.yaml 或 config.d/commands.yaml 中配置）
    #[serde(default)]
    pub commands: CommandMap,
}

impl Default for AxConfig {
    fn default() -> Self {
        Self {
            auto_sync: true,
            commands: CommandMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ProxyConfig {
    pub address: String,
    pub no_proxy: String,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            address: "http://vpn.yushe.ai:7890".into(),
            no_proxy: "localhost,127.0.0.1,10.0.0.0/8,172.16.0.0/12,192.168.0.0/16,*.local".into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct ShellConfig {
    pub default: String,
    pub plugins: Vec<PluginEntry>,
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            default: "zsh".into(),
            plugins: vec![
                PluginEntry {
                    name: "zsh-autosuggestions".into(),
                    url: "https://github.com/zsh-users/zsh-autosuggestions".into(),
                },
                PluginEntry {
                    name: "zsh-syntax-highlighting".into(),
                    url: "https://github.com/zsh-users/zsh-syntax-highlighting".into(),
                },
                PluginEntry {
                    name: "zsh-completions".into(),
                    url: "https://github.com/zsh-users/zsh-completions".into(),
                },
            ],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginEntry {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct PackagesConfig {
    pub dir: String,
}

impl Default for PackagesConfig {
    fn default() -> Self {
        Self {
            dir: String::new(), // 运行时填充
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct DeployConfig {
    pub backup_dir: String,
    pub links: Vec<LinkEntry>,
}

impl Default for DeployConfig {
    fn default() -> Self {
        Self {
            backup_dir: String::new(), // 运行时填充
            links: vec![
                LinkEntry {
                    src: "wezterm/wezterm.lua".into(),
                    dst: "~/.config/wezterm/wezterm.lua".into(),
                    optional: false,
                },
                LinkEntry {
                    src: "git/.gitconfig".into(),
                    dst: "~/.gitconfig".into(),
                    optional: true,
                },
                LinkEntry {
                    src: "tmux/tmux.conf".into(),
                    dst: "~/.config/tmux/tmux.conf".into(),
                    optional: true,
                },
            ],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LinkEntry {
    pub src: String,
    pub dst: String,
    #[serde(default)]
    pub optional: bool,
}

/// 命令库（JSON）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandEntry {
    pub cmd: String,
    #[serde(default)]
    pub desc: String,
}

pub type CommandMap = BTreeMap<String, CommandEntry>;

/// 环境变量条目
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnvEntry {
    pub value: String,
    #[serde(default)]
    pub desc: String,
    /// 标签分组（用于批量操作）
    #[serde(default)]
    pub tags: Vec<String>,
    /// 是否暂停
    #[serde(default)]
    pub paused: bool,
}

pub type EnvMap = BTreeMap<String, EnvEntry>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum SshAuth {
    Key,
    Password,
}

impl Default for SshAuth {
    fn default() -> Self {
        Self::Key
    }
}

impl SshAuth {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Key => "key",
            Self::Password => "password",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct SshHostEntry {
    pub host: String,
    pub user: String,
    #[serde(default = "default_ssh_port")]
    pub port: u16,
    pub auth: SshAuth,
    pub password: String,
    pub key: String,
    pub desc: String,
}

pub type SshHostMap = BTreeMap<String, SshHostEntry>;

fn default_ssh_port() -> u16 {
    22
}

impl Default for SshHostEntry {
    fn default() -> Self {
        Self {
            host: String::new(),
            user: String::new(),
            port: default_ssh_port(),
            auth: SshAuth::Key,
            password: String::new(),
            key: String::new(),
            desc: String::new(),
        }
    }
}

/// 合并主配置的命令和 config.d/commands.yaml 的命令
pub fn load_all_commands(config: &Config) -> Result<CommandMap> {
    let mut map = config.ax.commands.clone();
    let path = config_dir().join("config.d").join("commands.yaml");
    if path.exists() {
        let content = std::fs::read_to_string(&path)?;
        let extra: CommandMap = serde_yaml::from_str(&content).unwrap_or_default();
        for (k, v) in extra {
            map.insert(k, v);
        }
    }
    Ok(map)
}

/// 保存命令到 config.d/commands.yaml
pub fn save_commands(map: &CommandMap) -> Result<()> {
    let cdir = config_dir();
    std::fs::create_dir_all(cdir.join("config.d"))?;
    let content = serde_yaml::to_string(map)?;
    std::fs::write(cdir.join("config.d").join("commands.yaml"), content)?;
    Ok(())
}

/// 根据 commands.yaml 生成 shell 函数文件（供 source 加载）
/// 每个命令生成同名函数，内联命令内容，使 cd 等内置命令在当前 shell 生效
pub fn generate_command_functions(config: &Config) -> Result<()> {
    let map = load_all_commands(config)?;
    let cdir = config_dir();
    std::fs::create_dir_all(cdir.join("config.d"))?;
    let path = cdir.join("config.d").join("commands.sh");

    if map.is_empty() {
        // 无命令时写入空文件，避免 source 报错
        std::fs::write(&path, "")?;
        return Ok(());
    }

    let mut lines = String::from("# ax-cli 自定义命令（自动生成，请勿手动编辑）\n\n");
    let mut entries: Vec<_> = map.iter().collect();
    entries.sort_by_key(|(k, _)| *k);

    for (name, entry) in &entries {
        // 命令名只允许安全字符，防止注入
        if !name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            eprintln!("⚠️  跳过无效命令名: {name}（仅允许字母、数字、- 和 _）");
            continue;
        }
        // 用 eval + 单引号包裹命令内容，避免 source 时执行替换
        let escaped = entry.cmd.replace('\'', "'\\''");
        lines.push_str(name);
        lines.push_str("() {\n  eval '");
        lines.push_str(&escaped);
        lines.push_str("'\n}\n\n");
    }

    std::fs::write(&path, lines)?;
    Ok(())
}

/// 加载环境变量（合并主配置 + config.d/env.yaml）
pub fn load_all_env(config: &Config) -> Result<EnvMap> {
    let mut map = config.env.clone();
    let path = config_dir().join("config.d").join("env.yaml");
    if path.exists() {
        let content = std::fs::read_to_string(&path)?;
        let extra: EnvMap = serde_yaml::from_str(&content).unwrap_or_default();
        for (k, v) in extra {
            map.insert(k, v);
        }
    }
    Ok(map)
}

/// 保存环境变量到 config.d/env.yaml
pub fn save_env(map: &EnvMap) -> Result<()> {
    let cdir = config_dir();
    std::fs::create_dir_all(cdir.join("config.d"))?;
    let content = serde_yaml::to_string(map)?;
    std::fs::write(cdir.join("config.d").join("env.yaml"), content)?;
    Ok(())
}

/// 加载 SSH 主机配置
pub fn load_all_ssh_hosts(_config: &Config) -> Result<SshHostMap> {
    let path = config_dir().join("config.d").join("ssh.yaml");
    if !path.exists() {
        return Ok(SshHostMap::new());
    }

    let content = std::fs::read_to_string(&path)?;
    let hosts: SshHostMap = serde_yaml::from_str(&content).unwrap_or_default();
    Ok(hosts)
}

/// 保存 SSH 主机配置到 config.d/ssh.yaml
pub fn save_ssh_hosts(map: &SshHostMap) -> Result<()> {
    let cdir = config_dir();
    std::fs::create_dir_all(cdir.join("config.d"))?;
    let content = serde_yaml::to_string(map)?;
    std::fs::write(cdir.join("config.d").join("ssh.yaml"), content)?;
    Ok(())
}

// ============ 配置目录 ============

/// 获取 ax-cli 配置根目录
/// Linux/macOS: ~/.config/axconfig/
/// Windows: %APPDATA%/axconfig/
pub fn config_dir() -> PathBuf {
    if let Ok(env_dir) = std::env::var("AX_CONFIG_DIR") {
        return expand_home(&env_dir);
    }

    #[cfg(target_os = "windows")]
    {
        if let Some(appdata) = dirs::data_dir() {
            return appdata.join("axconfig");
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        if let Some(config) = dirs::config_dir() {
            return config.join("axconfig");
        }
    }

    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config")
        .join("axconfig")
}

/// 获取二进制所在目录
fn binary_dir() -> Option<PathBuf> {
    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|p| p.to_path_buf()))
}

// ============ 配置加载器 ============

pub struct ConfigLoader;

impl ConfigLoader {
    /// 加载配置，按优先级合并
    pub fn load() -> Result<Config> {
        let mut merged: BTreeMap<String, serde_yaml::Value> = BTreeMap::new();

        // 1. 用户级配置（最低优先级）: ~/.config/axconfig/config.yaml + config.d/*.yaml
        let user_dir = config_dir();
        if let Ok(entries) = Self::load_yaml_dir(&user_dir.join("config.d")) {
            Self::merge_into(&mut merged, entries);
        }
        let user_file = user_dir.join("config.yaml");
        if user_file.exists() {
            if let Ok(entries) = Self::load_yaml_file(&user_file) {
                Self::merge_into(&mut merged, entries);
            }
        }

        // 2. 二进制同级目录（便携模式）
        if let Some(bin_dir) = binary_dir() {
            let portable_dir = bin_dir.join("config");
            if portable_dir.is_dir() {
                if let Ok(entries) = Self::load_yaml_dir(&portable_dir) {
                    Self::merge_into(&mut merged, entries);
                }
            }
            let portable_file = bin_dir.join("config.yaml");
            if portable_file.exists() {
                if let Ok(entries) = Self::load_yaml_file(&portable_file) {
                    Self::merge_into(&mut merged, entries);
                }
            }
        }

        // 3. 环境变量（最高优先级）
        if let Ok(env_dir) = std::env::var("AX_CONFIG_DIR") {
            let dir = expand_home(&env_dir);
            if dir.is_dir() {
                if let Ok(entries) = Self::load_yaml_dir(&dir) {
                    Self::merge_into(&mut merged, entries);
                }
            }
            let file = dir.join("config.yaml");
            if file.exists() {
                if let Ok(entries) = Self::load_yaml_file(&file) {
                    Self::merge_into(&mut merged, entries);
                }
            }
        }

        // 反序列化
        let config: Config = if merged.is_empty() {
            Config::default()
        } else {
            serde_yaml::from_value(serde_yaml::Value::Mapping(
                merged
                    .into_iter()
                    .map(|(k, v)| (serde_yaml::Value::String(k), v))
                    .collect(),
            ))?
        };

        // 填充运行时默认路径
        let config = Self::fill_defaults(config);

        Ok(config)
    }

    /// 填充未配置的路径默认值
    fn fill_defaults(mut config: Config) -> Config {
        let cdir = config_dir();

        if config.packages.dir.is_empty() {
            config.packages.dir = cdir.join("packages").display().to_string();
        }

        if config.deploy.backup_dir.is_empty() {
            config.deploy.backup_dir = format!("~/.ax-backup-$(date +%Y%m%d%H%M%S)");
        }

        config
    }

    /// 加载目录下所有 yaml 文件
    fn load_yaml_dir(dir: &Path) -> Result<BTreeMap<String, serde_yaml::Value>> {
        let mut merged = BTreeMap::new();
        if !dir.is_dir() {
            return Ok(merged);
        }

        let mut files: Vec<_> = std::fs::read_dir(dir)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .map(|ext| ext == "yaml" || ext == "yml")
                    .unwrap_or(false)
            })
            .collect();
        files.sort_by_key(|e| e.file_name());

        for entry in files {
            if let Ok(entries) = Self::load_yaml_file(&entry.path()) {
                Self::merge_into(&mut merged, entries);
            }
        }
        Ok(merged)
    }

    /// 加载单个 yaml 文件
    fn load_yaml_file(path: &Path) -> Result<BTreeMap<String, serde_yaml::Value>> {
        let content = std::fs::read_to_string(path)?;
        let map: BTreeMap<String, serde_yaml::Value> = serde_yaml::from_str(&content)?;
        Ok(map)
    }

    /// 深度合并
    fn merge_into(
        base: &mut BTreeMap<String, serde_yaml::Value>,
        overlay: BTreeMap<String, serde_yaml::Value>,
    ) {
        for (key, value) in overlay {
            if let Some(base_val) = base.get(&key) {
                if let (
                    serde_yaml::Value::Mapping(base_map),
                    serde_yaml::Value::Mapping(overlay_map),
                ) = (base_val, &value)
                {
                    let mut merged_map = serde_yaml::Mapping::new();
                    for (bk, bv) in base_map {
                        merged_map.insert(bk.clone(), bv.clone());
                    }
                    for (ok, ov) in overlay_map {
                        merged_map.insert(ok.clone(), ov.clone());
                    }
                    base.insert(key, serde_yaml::Value::Mapping(merged_map));
                    continue;
                }
            }
            base.insert(key, value);
        }
    }
}

// ============ 工具函数 ============

/// 展开路径中的 ~
pub fn expand_home(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(&path[2..]);
        }
    }
    PathBuf::from(path)
}

// ============ 内置模板（从 config/ 目录嵌入） ============

pub const TEMPLATE_ZSHRC: &str = include_str!("../config/bash/.zshrc");
pub const TEMPLATE_BASHRC: &str = include_str!("../config/bash/.bashrc");
pub const TEMPLATE_WEZTERM: &str = include_str!("../config/wezterm/wezterm.lua");
pub const TEMPLATE_TMUX: &str = include_str!("../config/tmux/tmux.conf");

pub const TEMPLATE_CONFIG_YAML: &str = include_str!("../config/config.yaml");

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn uses_ax_config_dir_env_override() {
        let _guard = env_lock().lock().unwrap();
        let original = std::env::var("AX_CONFIG_DIR").ok();

        std::env::set_var("AX_CONFIG_DIR", "~/custom-axconfig");
        assert_eq!(config_dir(), expand_home("~/custom-axconfig"));

        match original {
            Some(value) => std::env::set_var("AX_CONFIG_DIR", value),
            None => std::env::remove_var("AX_CONFIG_DIR"),
        }
    }

    #[test]
    fn template_defaults_use_axconfig_directory() {
        assert!(TEMPLATE_CONFIG_YAML.contains("~/.config/axconfig/"));
        assert!(TEMPLATE_CONFIG_YAML.contains("~/.config/axconfig/packages"));
    }

    #[test]
    fn zsh_template_loads_generated_completion_dirs() {
        assert!(TEMPLATE_ZSHRC.contains("~/.zsh/completions"));
        assert!(TEMPLATE_ZSHRC.contains("site-functions"));
        assert!(!TEMPLATE_ZSHRC.contains("~/.ax/bash/completions/ax"));
    }

    #[test]
    fn bash_template_loads_generated_completion_file() {
        assert!(TEMPLATE_BASHRC.contains("bash-completion/completions/ax"));
        assert!(!TEMPLATE_BASHRC.contains("~/.ax/bash/completions/ax"));
    }
}
