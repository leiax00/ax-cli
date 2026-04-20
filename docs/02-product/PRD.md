# 产品需求文档 (PRD)

## 1. 产品定位

**ax-cli** 是一个面向个人开发者的 CLI 工具，用于统一管理开发环境的初始化、配置同步、工具安装和自定义命令。

核心价值：**单二进制 + 零运行时依赖 + 配置与工具分离 + 多机同步**。

## 2. 目标用户

- 需要在多台机器间同步开发环境的个人开发者
- 频繁重装系统或切换工作站的工程师
- 偏好 CLI 工作流、追求环境配置可复现的用户

## 3. 核心功能

### 3.1 环境初始化 (`ax install`)

一键完成开发环境搭建，包括：

- 系统包安装（按 OS 自动选择包管理器和包列表）
- Shell 安装与配置（zsh + 插件）
- 开发工具安装（fzf、starship、Nerd Font）
- 配置文件部署（symlink 方式）
- 自动备份已有配置

### 3.2 配置管理 (`ax config`)

- `init`：初始化配置目录，生成模板文件，创建 git 仓库
- `remote`：设置/查看远程仓库地址
- `push` / `pull`：通过 git 同步配置
- `export` / `import`：打包导出/导入配置（tar.gz）
- `path`：显示配置目录路径

### 3.3 命令管理 (`ax add/edit/rm/list/run`)

- 用户自定义快捷命令，存储在 `commands.yaml`
- 支持描述、分类
- 交互式选择执行

### 3.4 环境变量管理 (`ax env`)

- `add` / `edit` / `rm`：增删改环境变量
- `show`：查看变量（支持按名称/标签过滤）
- `tags`：标签分组管理
- `pause` / `resume`：临时禁用/恢复变量
- `load`：输出 shell 可 source 的 export 语句
- 多 shell 支持（bash、zsh、PowerShell、CMD）

### 3.5 代理管理 (`ax proxy`)

- `on` / `off`：启用/禁用 HTTP/HTTPS 代理
- `status`：查看当前代理状态
- 自动生成对应 shell 的环境变量设置命令

### 3.6 信息查看 (`ax info`)

- 显示配置目录路径、OS 信息、包管理器、Shell 类型
- 显示 git 远程仓库状态

### 3.7 Shell 补全 (`ax completion`)

- 生成 bash / zsh / PowerShell 补全脚本
- 补全基于真实命令树生成，支持多层子命令
- zsh 可显示命令与参数说明
- `ax install` 默认自动安装 zsh / bash 补全

### 3.8 Help 与补全多语言

- `ax --help` 与补全说明支持中文 / 英文
- 自动读取 `AX_LANG`、`LC_ALL`、`LC_MESSAGES`、`LANG`
- 未设置时默认中文

## 4. 多平台支持

| 平台 | 包管理器 | 状态 |
|------|---------|------|
| Ubuntu / Debian | apt | 支持 |
| Fedora / RHEL | dnf | 支持 |
| Arch Linux | pacman | 支持 |
| macOS | brew | 支持 |
| Windows | - | 基础支持 |

交叉编译目标：x86_64/aarch64 × Linux/macOS/Windows

## 5. 非功能需求

### 5.1 可靠性

- 所有操作幂等：重复执行不产生副作用
- 安装前自动备份已有配置文件
- 跳过已安装的包和工具

### 5.2 可移植性

- 单一二进制，零运行时依赖
- 支持便携模式：配置目录可与二进制同级
- 配置优先级：`AX_CONFIG_DIR` 环境变量 > 便携模式 > 默认路径

### 5.3 可维护性

- 模块化架构，命令间低耦合
- 新增命令只需三步：cli.rs 枚举 → commands/ 实现 → main.rs 路由
- 新增 OS 支持：添加 `packages/{os_id}.txt` 即可

## 6. 约束与假设

- Rust edition 2021，使用 stable 工具链
- 依赖保持精简：clap、serde、serde_yaml、anyhow、dirs、which、git2、regex、log
- 允许为 CLI 补全引入 `clap_complete`
- 不引入数据库或守护进程
- 配置格式固定为 YAML
