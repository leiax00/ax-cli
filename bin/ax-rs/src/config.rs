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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AxConfig {
    pub commands_file: String,
    pub auto_sync: bool,
    pub repo_dir: String,
}

impl Default for AxConfig {
    fn default() -> Self {
        Self {
            commands_file: "~/.ax-commands.json".into(),
            auto_sync: true,
            repo_dir: "~/.dotfiles".into(),
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
            dir: "~/.dotfiles/packages".into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct DeployConfig {
    pub dotfiles_dir: String,
    pub backup_dir: String,
    pub links: Vec<LinkEntry>,
}

impl Default for DeployConfig {
    fn default() -> Self {
        Self {
            dotfiles_dir: "~/.dotfiles".into(),
            backup_dir: "~/.dotfiles-backup".into(),
            links: vec![
                LinkEntry {
                    src: "bash/.zshrc".into(),
                    dst: "~/.zshrc".into(),
                    optional: false,
                },
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

/// 展开路径中的 ~ 为用户主目录
fn expand_home(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(&path[2..]);
        }
    }
    PathBuf::from(path)
}

/// 配置加载器
pub struct ConfigLoader;

impl ConfigLoader {
    /// 加载配置，按优先级合并
    pub fn load() -> Result<Config> {
        let mut merged: BTreeMap<String, serde_yaml::Value> = BTreeMap::new();

        // 1. 用户级配置目录
        let user_dirs = vec![
            dirs::config_dir().map(|d| d.join("ax")),
            dirs::home_dir().map(|d| d.join(".ax")),
        ];

        for dir_opt in user_dirs.iter().flatten() {
            if let Ok(entries) = Self::load_yaml_dir(dir_opt) {
                Self::merge_into(&mut merged, entries);
            }
            let file = dir_opt.join("config.yaml");
            if file.exists() {
                if let Ok(entries) = Self::load_yaml_file(&file) {
                    Self::merge_into(&mut merged, entries);
                }
            }
        }

        // 2. 可执行文件所在目录
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                let config_dir = exe_dir.join("config");
                if config_dir.is_dir() {
                    if let Ok(entries) = Self::load_yaml_dir(&config_dir) {
                        Self::merge_into(&mut merged, entries);
                    }
                }
                let config_file = exe_dir.join("config.yaml");
                if config_file.exists() {
                    if let Ok(entries) = Self::load_yaml_file(&config_file) {
                        Self::merge_into(&mut merged, entries);
                    }
                }
            }
        }

        // 3. 环境变量指定目录（最高优先级）
        if let Ok(env_dir) = std::env::var("AX_CONFIG_DIR") {
            let dir = expand_home(&env_dir);
            if dir.is_dir() {
                if let Ok(entries) = Self::load_yaml_dir(&dir) {
                    Self::merge_into(&mut merged, entries);
                }
            }
            let file = PathBuf::from(&env_dir).join("config.yaml");
            if file.exists() {
                if let Ok(entries) = Self::load_yaml_file(&file) {
                    Self::merge_into(&mut merged, entries);
                }
            }
        }

        // 反序列化为 Config
        let config: Config = serde_yaml::from_value(serde_yaml::Value::Mapping(merged.into_iter()
            .map(|(k, v)| (serde_yaml::Value::String(k), v))
            .collect()))?;

        Ok(config)
    }

    /// 加载目录下所有 yaml 文件，按文件名排序后合并
    fn load_yaml_dir(dir: &Path) -> Result<BTreeMap<String, serde_yaml::Value>> {
        let mut merged = BTreeMap::new();
        let mut files: Vec<_> = std::fs::read_dir(dir)?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path().extension().map(|ext| ext == "yaml" || ext == "yml").unwrap_or(false)
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

    /// 合并两个 map（后面的覆盖前面的）
    fn merge_into(
        base: &mut BTreeMap<String, serde_yaml::Value>,
        overlay: BTreeMap<String, serde_yaml::Value>,
    ) {
        for (key, value) in overlay {
            if let Some(base_val) = base.get(&key) {
                if let (serde_yaml::Value::Mapping(base_map), serde_yaml::Value::Mapping(overlay_map)) = (base_val, &value) {
                    // 深度合并 Mapping
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

/// 命令库（JSON）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandEntry {
    pub cmd: String,
    #[serde(default)]
    pub desc: String,
}

pub type CommandMap = BTreeMap<String, CommandEntry>;

pub struct CommandStore;

impl CommandStore {
    pub fn load(path: &Path) -> Result<CommandMap> {
        if !path.exists() {
            return Ok(CommandMap::new());
        }
        let content = std::fs::read_to_string(path)?;
        let map: CommandMap = serde_json::from_str(&content)?;
        Ok(map)
    }

    pub fn save(path: &Path, map: &CommandMap) -> Result<()> {
        let content = serde_json::to_string_pretty(map)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn add(map: &mut CommandMap, name: &str, cmd: &str, desc: &str) -> bool {
        if map.contains_key(name) {
            return false;
        }
        map.insert(name.into(), CommandEntry {
            cmd: cmd.into(),
            desc: desc.into(),
        });
        true
    }

    pub fn remove(map: &mut CommandMap, name: &str) -> bool {
        map.remove(name).is_some()
    }

    pub fn get<'a>(map: &'a CommandMap, name: &str) -> Option<&'a CommandEntry> {
        map.get(name)
    }
}
