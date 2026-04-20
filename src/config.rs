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

// ============ 内置模板 ============

pub const TEMPLATE_ZSHRC: &str = r#"# === ax-cli generated zsh config ===
# DO NOT EDIT MANUALLY - use 'ax config edit' or modify config.yaml

# === PATH ===
export PATH="$HOME/.local/bin:$PATH"

# === ax shell integration ===
ax() {
  if [ "$1" = "proxy" ] && { [ "$2" = "on" ] || [ "$2" = "off" ]; }; then
    eval "$(command ax "$@")"
  else
    command ax "$@"
  fi
}

# === 历史记录 ===
export HISTSIZE=50000
export HISTIGNORE="ls:ll:cd:pwd:clear:history"
setopt HIST_IGNORE_ALL_DUPS
setopt HIST_SAVE_NO_DUPS
setopt SHARE_HISTORY
setopt INC_APPEND_HISTORY

# === 补全 ===
fpath=("$HOME/.zsh/completions" "$HOME/.local/share/zsh/site-functions" $fpath)
[ -d "$HOME/.zsh/plugins/zsh-completions/src" ] && fpath=("$HOME/.zsh/plugins/zsh-completions/src" $fpath)
autoload -Uz compinit && compinit
zstyle ':completion:*' menu select
zstyle ':completion:*' matcher-list 'm:{a-zA-Z}={A-Za-z}'
setopt AUTO_LIST
setopt AUTO_MENU
setopt COMPLETE_IN_WORD

# === zsh plugins ===
[ -f "$HOME/.zsh/plugins/zsh-autosuggestions/zsh-autosuggestions.zsh" ] && source "$HOME/.zsh/plugins/zsh-autosuggestions/zsh-autosuggestions.zsh"
[ -f "$HOME/.zsh/plugins/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh" ] && source "$HOME/.zsh/plugins/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh"

# === Proxy (via ax) ===
pn() { ax proxy on "$1"; }
pf() { ax proxy off; }
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

pub const TEMPLATE_BASHRC: &str = r#"# === ax-cli generated bash config ===
# DO NOT EDIT MANUALLY - use 'ax config edit' or modify config.yaml

# === PATH ===
export PATH="$HOME/.local/bin:$PATH"

# === ax shell integration ===
ax() {
  if [ "$1" = "proxy" ] && { [ "$2" = "on" ] || [ "$2" = "off" ]; }; then
    eval "$(command ax "$@")"
  else
    command ax "$@"
  fi
}

# === 历史记录 ===
export HISTSIZE=50000
export HISTIGNORE="ls:ll:cd:pwd:clear:history"
shopt -s histappend
PROMPT_COMMAND="history -a; history -c; history -r; $PROMPT_COMMAND"

# === completion ===
if [ -f /usr/share/bash-completion/bash_completion ]; then
  source /usr/share/bash-completion/bash_completion
elif [ -f /etc/bash_completion ]; then
  source /etc/bash_completion
elif [ -f "$HOME/.local/share/bash-completion/bash_completion" ]; then
  source "$HOME/.local/share/bash-completion/bash_completion"
fi

[ -f "$HOME/.local/share/bash-completion/completions/ax" ] && source "$HOME/.local/share/bash-completion/completions/ax"

# === fzf ===
[ -f ~/.fzf.bash ] && source ~/.fzf.bash

# === Starship Prompt ===
if command -v starship &>/dev/null; then
  eval "$(starship init bash)"
fi

# === 常用 alias ===
alias ll='ls -alF'
alias la='ls -A'
alias l='ls -CF'
alias ..='cd ..'
alias ...='cd ../..'
alias grep='grep --color=auto'
alias egrep='egrep --color=auto'
alias fgrep='fgrep --color=auto'
alias cls='clear'

# === Proxy (via ax) ===
pn() { ax proxy on "$1"; }
pf() { ax proxy off; }
ps() { ax proxy status; }
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
# 优先级: AX_CONFIG_DIR > 可执行文件同级 config/ > ~/.config/axconfig/

ax:
  auto_sync: true
  commands:
    esp:
      cmd: "cd ~/esp/esp-idf && . export.sh"
      desc: "进入 ESP-IDF 开发环境"
    dcup:
      cmd: "docker compose up -d"
      desc: "启动 Docker 容器"
    dcdown:
      cmd: "docker compose down"
      desc: "停止 Docker 容器"
    dps:
      cmd: "docker ps --format 'table {{.Names}}\t{{.Status}}\t{{.Ports}}'"
      desc: "查看运行中的容器"
    lg:
      cmd: "lazygit"
      desc: "打开 lazygit"
    clean:
      cmd: "docker system prune -af"
      desc: "清理 Docker"
    ps:
      cmd: "ax proxy status"
      desc: "查看代理状态"

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
  dir: ~/.config/axconfig/packages

deploy:
  links:
    - src: wezterm/wezterm.lua
      dst: ~/.config/wezterm/wezterm.lua
    - src: git/.gitconfig
      dst: ~/.gitconfig
      optional: true

repo:
  remote: https://anyhub.yushe.ai/leiax00/ax-system-basic.git
  local_dir: ~/.ax-repo
"#;

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
}
