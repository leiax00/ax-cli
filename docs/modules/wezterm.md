# WezTerm

## 配置文件

`~/.dotfiles/wezterm/wezterm.lua`

部署后链接到：`~/.config/wezterm/wezterm.lua`

## 外观

- 主题：Catppuccin Mocha
- 字体：JetBrains Mono Nerd Font 14pt
- 背景：92% 透明度
- 标签栏：底部，仅多标签时显示
- 光标：闪烁竖线

## 快捷键

Leader 键：`Ctrl+A`（与 tmux 一致）

### 标签页

| 快捷键 | 功能 |
|--------|------|
| `Ctrl+A c` | 新建标签页 |
| `Ctrl+A n` | 下一个标签 |
| `Ctrl+A p` | 上一个标签 |
| `Ctrl+A 1-5` | 跳转到第 N 个标签 |
| `Ctrl+A w` | 关闭标签页 |

### 分屏

| 快捷键 | 功能 |
|--------|------|
| `Ctrl+A \|` | 水平分屏 |
| `Ctrl+A -` | 垂直分屏 |
| `Ctrl+A h/j/k/l` | 切换面板 |
| `Ctrl+A H/J/K/L` | 调整面板大小 |
| `Ctrl+A z` | 全屏/恢复当前面板 |
| `Ctrl+A x` | 关闭面板 |

### 其他

| 快捷键 | 功能 |
|--------|------|
| `Ctrl+A [` | 进入复制模式 |
| `Ctrl+Shift+V` | 粘贴 |
| `Ctrl+A u/d` | 向上/下滚动半页 |
| `Ctrl+Shift+PageUp/Down` | 滚动整页 |
| `Ctrl+ =/-` | 放大/缩小字体 |
| `Ctrl+ 0` | 重置字体 |
| `Ctrl+A r` | 重载配置 |
| `Ctrl+A a` | 发送实际 Ctrl+A（tmux 兼容） |

### 鼠标

- 左键选中 → 复制到剪贴板
- 右键 → 粘贴

## 性能

- 前端：WebGpu
- 最大帧率：120fps
- Wayland：已启用

## 自定义

直接编辑 `~/.dotfiles/wezterm/wezterm.lua`，然后 `Ctrl+A r` 重载。

常用自定义项：
- `font_size`：字体大小
- `window_background_opacity`：透明度（0.0 - 1.0）
- `color_scheme`：主题（运行 `wezterm ls-fonts --list-builtin` 查看可用主题）
- `default_cursor_style`：光标样式（`BlinkingBar` / `BlinkingBlock` / `SteadyUnderline`）

---

**返回** → [模块列表](./README.md)
