# ax - 自定义命令管理器

## 设计目标

解决 shell alias 的痛点：无法查看、无法搜索、无法补全、无法多机同步。

## 基本用法

```bash
ax add <名称> <命令> [描述]    # 添加命令
ax edit <名称>                  # 编辑命令
ax list                        # 列出所有命令
ax rm <名称>                   # 删除命令
ax <名称>                      # 执行命令
ax                             # fzf 交互选择（带预览）
ax help                        # 帮助
```

## 示例

```bash
ax add esp "cd ~/esp/esp-idf && . export.sh" "进入ESP-IDF环境"
ax add dcup "docker compose up -d" "启动Docker容器"
ax add lg "lazygit" "打开lazygit"

ax list
# 📋 自定义命令列表：
# ──────────────────────────────────────────
#   esp     进入ESP-IDF环境    → cd ~/esp/esp-idf && . export.sh
#   dcup    启动Docker容器     → docker compose up -d
#   lg      打开lazygit        → lazygit

ax esp          # 直接执行
ax             # 弹出 fzf 选择
```

## 自动补全

- **zsh**：输入 `ax ` 按 Tab，显示子命令 + 已有命令名（带描述）
- **bash**：输入 `ax ` 按两下 Tab，列出所有可用命令

## 自动同步

`ax add/edit/rm` 操作后自动：
1. 将 `~/.ax-commands.json`（符号链接到仓库）的变更写入 `~/.dotfiles/ax-commands.json`
2. 在 dotfiles 仓库中 `git commit`
3. 后台 `git push` 到远程

> 新机器 `git pull` 即可获得所有命令。

## 存储

- 命令库：`~/.dotfiles/ax-commands.json`
- 链接：`~/.ax-commands.json` → `~/.dotfiles/ax-commands.json`
- 格式：JSON，结构如下

```json
{
  "esp": {
    "cmd": "cd ~/esp/esp-idf && . export.sh",
    "desc": "进入ESP-IDF环境"
  }
}
```

## 关闭自动同步

编辑 `~/.dotfiles/bin/ax`，将 `AUTO_SYNC=true` 改为 `AUTO_SYNC=false`。

---

**返回** → [模块列表](./README.md)
