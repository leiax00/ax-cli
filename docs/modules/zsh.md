# zsh

## 配置文件

`~/.ax/bash/.zshrc`

部署后链接到：`~/.zshrc`

## 插件

手动安装（不用 Oh My Zsh），位于 `~/.zsh/plugins/`：

### zsh-autosuggestions

根据历史记录实时提示命令，灰色显示，按 `→` 接受。

```bash
# 效果示例
# 输入 docker compose up，会灰色提示上次的历史命令
# 按 → 直接采纳
```

### zsh-syntax-highlighting

命令实时语法高亮：
- ✅ 正确命令 → 绿色
- ❌ 错误命令 → 红色
- 🔵 参数 → 蓝色

### zsh-completions

增强补全，支持：
- git 分支名补全
- docker 容器名/镜像名补全
- npm/yarn 包名补全
- kill 信号名补全

## 历史记录

```bash
HISTSIZE=50000              # 保存 5 万条
HIST_IGNORE_ALL_DUPS        # 去重
SHARE_HISTORY               # 多终端共享
INC_APPEND_HISTORY          # 实时写入（不关闭终端也能共享）
```

## 补全配置

```bash
zstyle ':completion:*' menu select        # 交互式补全菜单
zstyle ':completion:*' matcher-list 'm:{a-zA-Z}={A-Za-z}'  # 大小写不敏感
```

## 键盘绑定

| 快捷键 | 功能 |
|--------|------|
| `↑/↓` | 搜索历史（匹配当前输入） |
| `→` | 接受 autosuggestion |
| `Tab` | 补全（zsh 原生） |

## 自定义

编辑 `~/.ax/bash/.zshrc`，修改后 `source ~/.zshrc` 生效。

---

**返回** → [模块列表](./README.md)
