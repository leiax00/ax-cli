# 快速参考

## ax 命令管理

```bash
ax add <名称> <命令> [描述]    # 添加
ax edit <名称>                  # 编辑
ax list                        # 列表
ax rm <名称>                   # 删除
ax <名称>                      # 执行
ax                             # fzf 选择
```

## WezTerm 快捷键（Leader: Ctrl+A）

### 标签页

```
Ctrl+A c          新建
Ctrl+A n / p      下/上一个
Ctrl+A 1-5        跳转
Ctrl+A w          关闭
```

### 分屏

```
Ctrl+A |          水平分屏
Ctrl+A -          垂直分屏
Ctrl+A h/j/k/l    切换面板
Ctrl+A H/J/K/L    调整大小
Ctrl+A z          全屏切换
Ctrl+A x          关闭面板
```

### 其他

```
Ctrl+A [          复制模式
Ctrl+Shift+V      粘贴
Ctrl+A r          重载配置
Ctrl+A a          发送 Ctrl+A
Ctrl+ =/-         字体缩放
```

## zsh

```
→                 接受建议
↑/↓               搜索历史
Tab               补全
```

## 常用 alias

```bash
ll                 ls -alF
la                 ls -A
..                 cd ..
...                cd ../..
grep               grep --color=auto
cls                clear
```

## 部署

```bash
# 新机器
git clone https://anyhub.yushe.ai/leiax00/ax-system-basic.git ~/.dotfiles
~/.dotfiles/install.sh && exec zsh

# 更新
cd ~/.dotfiles && git pull && exec zsh
```

---

**返回** → [文档总览](./README.md)
