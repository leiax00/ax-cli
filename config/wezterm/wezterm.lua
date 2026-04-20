local wezterm = require 'wezterm'
local act = wezterm.action

-- === 主题 ===
local theme = 'Catppuccin Mocha'

local config = {
  -- 基础外观
  color_scheme = theme,
  font = wezterm.font 'JetBrains Mono Nerd Font',
  font_size = 10,
  initial_cols = 150,
  initial_rows = 36,
  line_height = 1.2,
  window_background_opacity = 0.92,
  text_background_opacity = 0.92,
  -- NONE可以隐藏header
  window_decorations = "RESIZE",
  hide_tab_bar_if_only_one_tab = false,
  enable_tab_bar = true,
  tab_bar_at_bottom = true,
  use_fancy_tab_bar = true,
  tab_max_width = 32,

  -- 性能
  max_fps = 120,
  front_end = "WebGpu",
  enable_wayland = true,

  -- 光标
  default_cursor_style = "BlinkingBar",
  cursor_blink_rate = 500,

  -- 滚动
  scrollback_lines = 10000,
  enable_scroll_bar = false,
}

-- === 快捷键 (Leader: Ctrl+A) ===
config.leader = { key = 'a', mods = 'CTRL', timeout_milliseconds = 5000 }

config.keys = {
  -- Leader + a → 发送 Ctrl+A (tmux 兼容)
  { key = 'a', mods = 'LEADER|CTRL', action = act.SendKey { key = 'a', mods = 'CTRL' } },

  -- 标签页
  { key = 't', mods = 'CTRL', action = act.ShowTabNavigator },
  { key = 'c', mods = 'LEADER', action = act.SpawnTab 'CurrentPaneDomain' },
  { key = 'n', mods = 'LEADER', action = act.ActivateTabRelative(1) },
  { key = 'p', mods = 'LEADER', action = act.ActivateTabRelative(-1) },
  { key = 'w', mods = 'LEADER', action = act.CloseCurrentTab { confirm = true } },
  { key = '1', mods = 'LEADER', action = act.ActivateTab(0) },
  { key = '2', mods = 'LEADER', action = act.ActivateTab(1) },
  { key = '3', mods = 'LEADER', action = act.ActivateTab(2) },
  { key = '4', mods = 'LEADER', action = act.ActivateTab(3) },
  { key = '5', mods = 'LEADER', action = act.ActivateTab(4) },

  -- 分屏
  { key = '\\', mods = 'LEADER', action = act.SplitHorizontal { domain = 'CurrentPaneDomain' } },
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

  -- 复制粘贴
  { key = '[', mods = 'LEADER', action = act.ActivateCopyMode },
  { key = 'v', mods = 'CTRL|SHIFT', action = act.PasteFrom 'Clipboard' },

  -- 滚动
  { key = 'PageUp', mods = 'SHIFT', action = act.ScrollByPage(-1) },
  { key = 'PageDown', mods = 'SHIFT', action = act.ScrollByPage(1) },
  { key = 'u', mods = 'LEADER', action = act.ScrollByPage(-0.5) },
  { key = 'd', mods = 'LEADER', action = act.ScrollByPage(0.5) },

  -- 字体大小
  { key = '=', mods = 'CTRL', action = act.IncreaseFontSize },
  { key = '-', mods = 'CTRL', action = act.DecreaseFontSize },
  { key = '0', mods = 'CTRL', action = act.ResetFontSize },

  -- 重载配置
  { key = 'r', mods = 'LEADER', action = act.ReloadConfiguration },
}

-- === 鼠标 ===
config.mouse_bindings = {
  { event = { Up = { streak = 1, button = 'Left' } }, mods = 'NONE', action = act.CompleteSelection 'ClipboardAndPrimarySelection' },
  { event = { Down = { streak = 1, button = 'Right' } }, mods = 'NONE', action = act.PasteFrom 'Clipboard' },
}

-- === 默认启动程序 ===
config.default_prog = { '/usr/bin/env', 'bash', '-l' }

-- === 状态栏 ===
wezterm.on('update-status', function(window, pane)
  window:set_right_status(wezterm.format {
    { Foreground = { Color = '#cdd6f4' } },
    { Background = { Color = '#1e1e2e' } },
    { Text = ' ' .. pane:get_current_working_dir() .. ' ' },
  })
end)

return config
