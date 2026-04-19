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
    pub repo: RepoConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct AxConfig {
    pub commands_file: String,
    pub auto_sync: bool,
}

impl Default for AxConfig {
    fn default() -> Self {
        Self {
            commands_file: String::new(), // 运行时填充
            auto_sync: true,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct RepoConfig {
    /// Git 远程仓库地址（可选，用于同步）
    pub remote: String,
    /// 本地仓库目录（默认 ~/.ax-repo）
    pub local_dir: String,
}

impl Default for RepoConfig {
    fn default() -> Self {
        Self {
            remote: "https://anyhub.yushe.ai/leiax00/ax-system-basic.git".into(),
            local_dir: "~/.ax-repo".into(),
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
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(map)?;
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

// ============ 配置目录 ============

/// 获取 ax-cli 配置根目录
/// Linux/macOS: ~/.config/ax-cli/
/// Windows: %APPDATA%/ax-cli/
pub fn config_dir() -> PathBuf {
    if let Ok(env_dir) = std::env::var("AX_CONFIG_DIR") {
        return expand_home(&env_dir);
    }

    #[cfg(target_os = "windows")]
    {
        if let Some(appdata) = dirs::data_dir() {
            return appdata.join("ax-cli");
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        if let Some(config) = dirs::config_dir() {
            return config.join("ax-cli");
        }
    }

    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".config")
        .join("ax-cli")
}

/// 获取二进制所在目录
fn binary_dir() -> Option<PathBuf> {
    std::env::current_exe().ok().and_then(|exe| exe.parent().map(|p| p.to_path_buf()))
}

// ============ 配置加载器 ============

pub struct ConfigLoader;

impl ConfigLoader {
    /// 加载配置，按优先级合并
    pub fn load() -> Result<Config> {
        let mut merged: BTreeMap<String, serde_yaml::Value> = BTreeMap::new();

        // 1. 用户级配置（最低优先级）: ~/.config/ax-cli/config.yaml + config.d/*.yaml
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
            serde_yaml::from_value(serde_yaml::Value::Mapping(merged.into_iter()
                .map(|(k, v)| (serde_yaml::Value::String(k), v))
                .collect()))?
        };

        // 填充运行时默认路径
        let config = Self::fill_defaults(config);

        Ok(config)
    }

    /// 填充未配置的路径默认值
    fn fill_defaults(mut config: Config) -> Config {
        let cdir = config_dir();

        if config.ax.commands_file.is_empty() {
            config.ax.commands_file = format!("~/ax-commands.json");
        }

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
                e.path().extension()
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
                if let (serde_yaml::Value::Mapping(base_map), serde_yaml::Value::Mapping(overlay_map)) = (base_val, &value) {
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

// ============ 内置模板 ============

pub const TEMPLATE_ZSHRC: &str = r#"# === ax-cli generated zsh config ===
# DO NOT EDIT MANUALLY - use 'ax config edit' or modify config.yaml

# === PATH ===
export PATH="$HOME/.local/bin:$PATH"

# === 历史记录 ===
export HISTSIZE=50000
export HISTIGNORE="ls:ll:cd:pwd:clear:history"
setopt HIST_IGNORE_ALL_DUPS
setopt HIST_SAVE_NO_DUPS
setopt SHARE_HISTORY
setopt INC_APPEND_HISTORY

# === 补全 ===
autoload -Uz compinit && compinit
zstyle ':completion:*' menu select
zstyle ':completion:*' matcher-list 'm:{a-zA-Z}={A-Za-z}'
setopt AUTO_LIST
setopt AUTO_MENU
setopt COMPLETE_IN_WORD

# === Proxy (via ax) ===
pn() { eval $(ax proxy on "$1") }
pf() { eval $(ax proxy off) }
ps() { ax proxy status }

# === fzf ===
[ -f ~/.fzf.zsh ] && source ~/.fzf.zsh

# === Starship Prompt ===
if command -v starship &>/dev/null; then
  eval "$(starship init zsh)"
fi

# === 常用 alias ===
alias ll='ls -alF'
alias la='ls -A'
alias l='ls -CF'
alias ..='cd ..'
alias ...='cd ../..'
alias grep='grep --color=auto'
alias cls='clear'

# === 键盘绑定 ===
bindkey -e
bindkey '^[[A' up-line-or-search
bindkey '^[[B' down-line-or-search
"#;

pub const TEMPLATE_WEZTERM: &str = r#"local wezterm = require 'wezterm'
local act = wezterm.action

local config = {
  color_scheme = 'Catppuccin Mocha',
  font = wezterm.font 'JetBrains Mono Nerd Font',
  font_size = 14,
  line_height = 1.2,
  window_background_opacity = 0.92,
  text_background_opacity = 0.92,
  window_decorations = "RESIZE",
  hide_tab_bar_if_only_one_tab = true,
  tab_bar_at_bottom = true,
  use_fancy_tab_bar = true,
  tab_max_width = 32,
  max_fps = 120,
  front_end = "WebGpu",
  enable_wayland = true,
  default_cursor_style = "BlinkingBar",
  cursor_blink_rate = 500,
  scrollback_lines = 10000,
  enable_scroll_bar = false,
}

config.leader = { key = 'a', mods = 'CTRL', timeout_milliseconds = 1000 }

config.keys = {
  { key = 'a', mods = 'LEADER|CTRL', action = act.SendKey { key = 'a', mods = 'CTRL' } },
  { key = 'c', mods = 'LEADER', action = act.SpawnTab 'CurrentPaneDomain' },
  { key = 'n', mods = 'LEADER', action = act.ActivateTabRelative(1) },
  { key = 'p', mods = 'LEADER', action = act.ActivateTabRelative(-1) },
  { key = 'w', mods = 'LEADER', action = act.CloseCurrentTab { confirm = true } },
  { key = '1', mods = 'LEADER', action = act.ActivateTab(0) },
  { key = '2', mods = 'LEADER', action = act.ActivateTab(1) },
  { key = '3', mods = 'LEADER', action = act.ActivateTab(2) },
  { key = '4', mods = 'LEADER', action = act.ActivateTab(3) },
  { key = '5', mods = 'LEADER', action = act.ActivateTab(4) },
  { key = '|', mods = 'LEADER|SHIFT', action = act.SplitHorizontal { domain = 'CurrentPaneDomain' } },
  { key = '-', mods = 'LEADER', action = act.SplitVertical { domain = 'CurrentPaneDomain' } },
  { key = 'h', mods = 'LEADER', action = act.ActivatePaneDirection 'Left' },
  { key = 'j', mods = 'LEADER', action = act.ActivatePaneDirection 'Down' },
  { key = 'k', mods = 'LEADER', action = act.ActivatePaneDirection 'Up' },
  { key = 'l', mods = 'LEADER', action = act.ActivatePaneDirection 'Right' },
  { key = 'H', mods = 'LEADER|SHIFT', action = act.AdjustPaneSize { 'Left', 5 } },
  { key = 'J', mods = 'LEADER|SHIFT', action = act.AdjustPaneSize { 'Down', 5 } },
  { key = 'K', mods = 'LEADER|SHIFT', action = act.AdjustPaneSize { 'Up', 5 } },
  { key = 'L', mods = 'LEADER|SHIFT', action = act.AdjustPaneSize { 'Right', 5 } },
  { key = 'z', mods = 'LEADER', action = act.TogglePaneZoomState },
  { key = 'x', mods = 'LEADER', action = act.CloseCurrentPane { confirm = true } },
  { key = '[', mods = 'LEADER', action = act.ActivateCopyMode },
  { key = 'v', mods = 'CTRL|SHIFT', action = act.PasteFrom 'Clipboard' },
  { key = 'PageUp', mods = 'SHIFT', action = act.ScrollByPage(-1) },
  { key = 'PageDown', mods = 'SHIFT', action = act.ScrollByPage(1) },
  { key = 'u', mods = 'LEADER', action = act.ScrollByPage(-0.5) },
  { key = 'd', mods = 'LEADER', action = act.ScrollByPage(0.5) },
  { key = '=', mods = 'CTRL', action = act.IncreaseFontSize },
  { key = '-', mods = 'CTRL', action = act.DecreaseFontSize },
  { key = '0', mods = 'CTRL', action = act.ResetFontSize },
  { key = 'r', mods = 'LEADER', action = act.ReloadConfiguration },
}

config.mouse_bindings = {
  { event = { Up = { streak = 1, button = 'Left' } }, mods = 'NONE', action = act.CompleteSelection 'ClipboardAndPrimarySelection' },
  { event = { Down = { streak = 1, button = 'Right' } }, mods = 'NONE', action = act.PasteFrom 'Clipboard' },
}

config.default_prog = { '/usr/bin/env', 'bash', '-l' }

wezterm.on('update-status', function(window, pane)
  window:set_right_status(wezterm.format {
    { Foreground = { Color = '#cdd6f4' } },
    { Background = { Color = '#1e1e2e' } },
    { Text = ' ' .. pane:get_current_working_dir() .. ' ' },
  })
end)

return config
"#;

pub const TEMPLATE_CONFIG_YAML: &str = r#"# ax-cli 配置文件
# 优先级: AX_CONFIG_DIR > 可执行文件同级 config/ > ~/.config/ax-cli/

ax:
  commands_file: ~/ax-commands.json
  auto_sync: true

proxy:
  address: "http://vpn.yushe.ai:7890"
  no_proxy: "localhost,127.0.0.1,10.0.0.0/8,172.16.0.0/12,192.168.0.0/16,*.local"

shell:
  default: zsh
  plugins:
    - name: zsh-autosuggestions
      url: https://github.com/zsh-users/zsh-autosuggestions
    - name: zsh-syntax-highlighting
      url: https://github.com/zsh-users/zsh-syntax-highlighting
    - name: zsh-completions
      url: https://github.com/zsh-users/zsh-completions

packages:
  dir: ~/.config/ax-cli/packages

deploy:
  links:
    - src: bash/.zshrc
      dst: ~/.zshrc
    - src: wezterm/wezterm.lua
      dst: ~/.config/wezterm/wezterm.lua
    - src: git/.gitconfig
      dst: ~/.gitconfig
      optional: true

repo:
  remote: https://anyhub.yushe.ai/leiax00/ax-system-basic.git
  local_dir: ~/.ax-repo
"#;
