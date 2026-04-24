# 终端快捷键体系：WezTerm + tmux 职责分离

## 设计决策

WezTerm（终端模拟器）和 tmux（终端复用器）都提供分屏和标签页功能，但职责不同。将两者的 Leader 键设为相同值会导致所有快捷键冲突。

**方案**：职责分离 + 不同 Leader 键，零冲突。

| 层 | 职责 | Leader 键 | 适用场景 |
|---|------|----------|---------|
| WezTerm | tab 管理 + 无 tmux 时的 pane | `Ctrl+Q` | 始终可用 |
| tmux | pane/window/session 管理 | `Ctrl+A` | 开启 tmux 后 |

### 为什么选 `Ctrl+Q`

- 与 tmux 的 `Ctrl+A` 完全不同，零冲突
- 单手可按，比 `Ctrl+Shift+*` 方便
- tmux 不会使用此组合
- macOS/Linux 均无系统级占用

## WezTerm 快捷键

Leader 键：`Ctrl+Q`，超时 5000ms。

### Tab 管理

| 快捷键 | 功能 | 说明 |
|--------|------|------|
| `Ctrl+Shift+T` | 新建 tab | 直接绑定 |
| `Ctrl+Shift+W` | 关闭 tab（有确认） | 直接绑定 |
| `Ctrl+Tab` | 下一个 tab | 直接绑定，浏览器通用习惯 |
| `Ctrl+Shift+Tab` | 上一个 tab | 直接绑定 |
| `Ctrl+1` ~ `Ctrl+5` | 跳转 tab 1-5 | 直接绑定 |
| `Leader + ,` | 重命名当前 tab | 交互式输入 |

### Pane 管理（Leader 后按，无 tmux 时使用）

| 快捷键 | 功能 |
|--------|------|
| `Leader + \` | 水平分屏 |
| `Leader + -` | 垂直分屏 |
| `Leader + h/j/k/l` | 切换 pane |
| `Leader + Shift+H/J/K/L` | 调整 pane 大小（5 格） |
| `Leader + z` | pane 最大化/还原 |
| `Leader + x` | 关闭 pane（有确认） |

### 其他

| 快捷键 | 功能 |
|--------|------|
| `Leader + r` | 重载配置 |
| `Ctrl+Shift+V` | 粘贴 |
| `Ctrl+Shift+C` | 复制选中内容 |
| `Shift+PageUp/Down` | 滚动 |
| `Ctrl+= / Ctrl+- / Ctrl+0` | 增大/减小/重置字体 |
| `F11` | 窗口最大化 |

## tmux 快捷键

Prefix 键：`Ctrl+A`，escape-time 为 0。

### 分屏

| 快捷键 | 功能 |
|--------|------|
| `Prefix + \|` | 水平分屏（当前路径） |
| `Prefix + -` | 垂直分屏（当前路径） |
| `Prefix + c` | 新建窗口（当前路径） |

### 面板导航

| 快捷键 | 功能 |
|--------|------|
| `Prefix + h/j/k/l` | 选择左/下/上/右 pane |
| `Prefix + Shift+H/J/K/L` | 调整 pane 大小（5 格，可重复） |
| `Prefix + z` | pane 缩放 |

### 复制模式（vi 风格）

| 快捷键 | 功能 |
|--------|------|
| `Prefix + [` | 进入复制模式 |
| 复制模式中 `v` | 开始选择 |
| 复制模式中 `y` | 复制并退出 |
| 复制模式中 `Escape` | 取消 |

### 其他

| 快捷键 | 功能 |
|--------|------|
| `Prefix + r` | 重载配置 |
| `Prefix + d` | detach 会话 |

## 冲突验证

| 操作 | 无 tmux | 有 tmux |
|------|--------|--------|
| `Ctrl+Q + \` | WezTerm 分屏 | WezTerm 分屏（不影响 tmux） |
| `Ctrl+A + \|` | 发送到终端 | tmux 分屏 |
| `Ctrl+Shift+T` | WezTerm 新 tab | WezTerm 新 tab |
| `Ctrl+Tab` | WezTerm 切 tab | WezTerm 切 tab |

## 使用场景

```
本地轻量开发：
  WezTerm tab 1 → 不开 tmux，用 WezTerm pane 分屏
  WezTerm tab 2 → 不开 tmux，用 WezTerm pane 分屏

远程 / 需要持久化：
  WezTerm tab 1 → tmux session（pane 全交给 tmux）
  WezTerm tab 2 → tmux session
```

## 相关文件

- WezTerm 配置模板：`config/wezterm/wezterm.lua`
- tmux 配置模板：`config/tmux/tmux.conf`
- 源码中的模板嵌入：`src/config.rs`（`TEMPLATE_WEZTERM`、`TEMPLATE_TMUX`）
