# 模块架构 (lib/)

## 设计理念

`install.sh` 和 `ax update` 共用同一套模块，避免逻辑重复。
每个模块职责单一，独立可测试，新增组件只需加一个文件。

## 模块一览

```
lib/
├── detect.sh       # 系统检测 + 包管理器选择
├── packages.sh     # 系统包安装/检查
├── shell.sh        # zsh + 插件安装/更新
├── tools.sh        # fzf / starship / 字体
└── deploy.sh       # 配置文件链接/备份
```

## detect.sh - 系统检测

自动检测 Linux 发行版，设置包管理器相关变量。

**输出变量：**

| 变量 | 说明 |
|------|------|
| `OS_ID` | 发行版 ID（ubuntu / fedora / arch ...） |
| `OS_NAME` | 发行版全名 |
| `PKG_MANAGER` | 包管理器名称 |
| `PKG_CHECK` | 检查包是否已安装的命令 |
| `PKG_UPDATE` | 更新包列表的命令 |
| `PKG_INSTALL` | 安装包的命令 |
| `PKG_LIST_FILE` | 对应的包列表文件路径 |

**新增发行版：**
1. 创建 `packages/<id>.txt`
2. 在 `detect.sh` 的 `_detect_pkg_manager()` 中加 case

## packages.sh - 系统包

提供 `install_packages()` 函数，读取对应发行版的包列表，只安装缺失的包。

**依赖：** detect.sh 设置的变量

## shell.sh - zsh + 插件

提供以下函数：

| 函数 | 用途 |
|------|------|
| `install_zsh()` | 安装 zsh |
| `set_default_shell()` | 切换默认 shell |
| `install_zsh_plugins()` | 首次安装插件 |
| `update_zsh_plugins()` | 更新已有插件 / 安装缺失插件 |

插件列表在 `ZSH_PLUGINS` 数组中维护，格式：`"名称|git地址"`。

## tools.sh - 通用工具

提供以下函数：

| 函数 | 用途 |
|------|------|
| `install_fzf()` | 安装 fzf |
| `install_starship()` | 安装 Starship |
| `install_nerd_font()` | 安装 JetBrains Mono Nerd Font |

## deploy.sh - 配置部署

提供以下函数：

| 函数 | 用途 |
|------|------|
| `backup()` | 备份已有文件 |
| `deploy_dotfiles()` | 链接 .zshrc / wezterm / .gitconfig |
| `deploy_ax_tool()` | 链接 ax + 命令库 |
| `update_ax_tool()` | 仅刷新 ax 工具链接 |

## 新增模块

```bash
# 1. 创建模块
cat > lib/mytool.sh << 'EOF'
install_mytool() {
  echo "安装 mytool..."
}

update_mytool() {
  echo "更新 mytool..."
}
EOF

# 2. 在 install.sh 中调用
source "$DOTDIR/lib/mytool.sh"
install_mytool

# 3. 在 ax update 中调用
source "$DOTDIR/lib/mytool.sh"
update_mytool
```

---

**返回** → [模块列表](./README.md)
