# 安装流程设计

## 概述

`ax install` 是核心命令，执行完整的开发环境搭建流水线。

## 流水线阶段

```
ax install
  │
  ├─ 1. 系统包安装 (check_and_install)
  │    ├─ 检测 OS → 选择包管理器
  │    ├─ 读取 packages/{os_id}.txt
  │    ├─ 解析 core / extras 分组
  │    └─ 跳过已安装的包，安装缺失包
  │
  ├─ 2. Shell 安装与配置 (install_zsh + set_default_shell)
  │    ├─ 检查 zsh 是否已安装
  │    ├─ 未安装则通过包管理器安装
  │    └─ 设置为默认 shell（需用户确认）
  │
  ├─ 3. Shell 插件安装 (install_plugins)
  │    ├─ 创建 ~/.zsh/plugins/ 目录
  │    ├─ 对每个插件：检查目录 → git clone
  │    └─ 插件列表来自 config.yaml 的 shell.plugins
  │
  ├─ 4. 开发工具安装
  │    ├─ fzf：git clone → install 脚本
  │    ├─ starship：curl 下载安装脚本 → 执行
  │    └─ Nerd Font：检查 fc-list → 下载解压 → fc-cache
  │
  ├─ 5. 配置文件部署 (deploy)
  │    ├─ 读取 config.yaml 的 deploy 列表
  │    ├─ 对每个条目：
  │    │   ├─ 检查目标文件是否存在
  │    │   ├─ 存在则备份（.bak 后缀）
  │    │   └─ 创建 symlink：source → target
  │    └─ source 相对于配置根目录
  │
  └─ 6. Shell 集成
       ├─ 刷新托管的 bash/.zshrc 和 bash/.bashrc
       ├─ 安装 zsh / bash 补全脚本
       ├─ 在 .zshrc / .bashrc 中添加 source 语句（如果不存在）
       └─ 确保 ax 命令与补全可用
```

## 幂等性保证

每个阶段都内置了跳过逻辑：

- **包安装**：`is_package_installed()` 检查后再决定是否安装
- **Shell**：`which zsh` 检查
- **插件**：目录存在则跳过 git clone
- **工具**：`which fzf` / `which starship` 检查
- **字体**：`fc-list` 匹配检查
- **部署**：symlink 已存在且指向正确则跳过
- **补全**：重复执行会覆盖补全脚本并刷新托管 shell 片段

## 包列表格式

```
# packages/{os_id}.txt

# === core ===
git
curl
zsh
unzip
fontconfig

# === extras ===
jq
fzf
wget
tree
htop
ripgrep
fd-find
bat
tmux
```

`# === core ===` 和 `# === extras ===` 作为分组标记。`ax install` 安装 core 组，`ax install --extras` 额外安装 extras 组。

## 相关模块

| 模块 | 职责 |
|------|------|
| `src/packages.rs` | 包列表解析、系统包安装 |
| `src/shell.rs` | zsh 安装、默认 shell 设置、插件管理 |
| `src/tools.rs` | fzf、starship、Nerd Font 安装 |
| `src/commands/install.rs` | 流水线编排 |
| `src/commands/completion.rs` | 生成并安装 shell 补全 |
