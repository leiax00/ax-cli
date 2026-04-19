# ax - 开发环境管理工具

## 设计目标

一个二进制解决：命令管理、环境变量、代理、配置同步、系统部署。

**核心理念：工具和配置分离。**

- `ax` 二进制 — 纯工具，不含任何配置
- `~/.config/ax-cli/` — 配置目录，独立 git 仓库，可同步、迁移、分享

---

## 配置管理

配置目录就是 git 仓库，初始化完直接能用。

```bash
ax config init              # 生成默认配置 + git init
ax config remote <url>      # 关联远程仓库
ax config push              # 推送配置
ax config pull              # 拉取配置
ax config path              # 显示配置目录
```

### 配置优先级

```
AX_CONFIG_DIR 环境变量          → 最高（临时切换配置）
  > 可执行文件同级 config/       → 便携模式（U盘、Windows）
    > ~/.config/ax-cli/         → 默认（XDG 标准）
```

### 配置文件

```
~/.config/ax-cli/
├── config.yaml            # 主配置（含默认命令、代理、shell 等）
├── config.d/              # 分块配置（自动与主配置合并）
│   ├── commands.yaml      # 用户动态增删的命令
│   └── env.yaml           # 用户动态增删的环境变量
├── bash/.zshrc
├── wezterm/wezterm.lua
├── packages/
└── git/.gitconfig
```

### 导入导出（迁移）

```bash
ax config export                 # 导出配置为 tar.gz
ax config export -f              # 含 ax 二进制的完整便携包
ax config export -o my-config.tar.gz  # 指定文件名

ax config import my-config.tar.gz     # 导入（不覆盖已有文件）
```

- 配置导出：跨平台迁移，只迁移配置
- 便携包（-f）：同类型系统直接解压即用

---

## 命令管理

解决 shell alias 的痛点：无法查看、无法搜索、无法补全、无法多机同步。

```bash
ax add <名称> <命令> [描述]    # 添加
ax edit <名称>                  # 编辑
ax list / ls                    # 列出
ax rm / del <名称>              # 删除
ax run [名称]                   # 执行（无参数时交互选择）
ax <名称>                       # 快捷执行
```

### 示例

```bash
ax add esp "cd ~/esp/esp-idf && . export.sh" "进入ESP-IDF环境"
ax add dcup "docker compose up -d" "启动Docker容器"
ax add lg "lazygit" "打开lazygit"

ax list
# 📋 自定义命令列表：
# ──────────────────────────────────────────
#   dcup    启动Docker容器     → docker compose up -d
#   lg      打开lazygit        → lazygit
#   esp     进入ESP-IDF环境    → cd ~/esp/esp-idf && . export.sh

ax esp          # 直接执行
```

### 自动同步

`ax add/edit/rm` 后自动 `git commit + push`（可通过 `auto_sync: false` 关闭）。

### Shell 补全

```bash
ax completion zsh          # 安装到 ~/.zsh/completions/_ax
ax completion bash         # 安装到 bash-completion 目录
ax completion powershell   # 追加到 PowerShell profile
ax completion zsh -p       # 只打印脚本（不安装）
```

---

## 环境变量管理

统一管理开发环境变量，支持标签分组和暂停。

```bash
# CRUD
ax env add GOPATH ~/go -t golang                    # 添加（-t 标签）
ax env add EDITOR vim -d "默认编辑器" -t "dev,common"  # 多标签
ax env edit GOPATH -v /new/path                     # 修改
ax env rm TEST_VAR                                  # 删除
ax env show                                         # 列表
ax env show -t docker                               # 按标签筛选

# 暂停/恢复
ax env pause DOCKER_HOST                            # 暂停单个
ax env pause -t golang                              # 暂停整个标签
ax env pause --all                                  # 暂停全部
ax env resume DOCKER_HOST                           # 恢复
ax env resume --all                                 # 恢复全部

# 加载到 shell（在 .zshrc 中使用）
eval $(ax env load)                                 # 只加载未暂停的变量

# 标签管理
ax env tags                                         # 查看所有标签
```

### 配置示例

```yaml
# config.d/env.yaml
GOPATH:
  value: "/home/user/go"
  desc: "Go 工作目录"
  tags: [golang]
  paused: false
DOCKER_HOST:
  value: "unix:///var/run/docker.sock"
  tags: [docker]
  paused: true       # 暂停后 ax env load 不会输出
EDITOR:
  value: vim
  tags: [dev, common]
```

---

## 代理管理

```bash
eval $(ax proxy on)                # 开启（输出 shell export）
eval $(ax proxy on http://other)   # 自定义地址
eval $(ax proxy off)               # 关闭
ax proxy status                    # 查看状态
```

### zsh 快捷方式

`.zshrc` 中内置了别名：

```bash
pn                              # 开启代理
pf                              # 关闭代理
ps                              # 查看状态
```

---

## 其他命令

```bash
ax info                         # 查看当前配置和路径
ax install                      # 一键安装（包+工具+部署配置）
ax push / pull                  # 配置同步快捷方式
ax completion <shell>           # 安装 shell 补全
ax help                         # 帮助
```

---

**返回** → [模块列表](./README.md)
