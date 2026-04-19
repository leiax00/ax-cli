# 快速参考

## ax 配置

```bash
ax config init              # 初始化配置 + git repo
ax config remote <url>      # 设置远程仓库
ax config push / pull       # 同步
ax config export [-f]       # 导出（-f 含二进制）
ax config import <file>     # 导入
ax config path              # 显示配置目录
```

## ax 命令管理

```bash
ax add <名> <命令> [描述]    # 添加
ax edit <名>                  # 编辑
ax list / ls                  # 列表
ax rm / del <名>              # 删除
ax run [名]                   # 执行
ax <名>                       # 快捷执行
```

## ax 环境变量

```bash
ax env add <名> <值> [-d 描述] [-t 标签]   # 添加
ax env edit <名> [-v 值] [-d 描述] [-t 标签] # 修改
ax env rm <名>...                           # 删除
ax env show [--all] [-t 标签]               # 列表
ax env pause <名> / -t <标签> / --all       # 暂停
ax env resume <名> / -t <标签> / --all      # 恢复
eval $(ax env load)                         # 加载到 shell
ax env tags                                 # 查看标签
```

## 代理

```bash
pn                              # eval $(ax proxy on)
pf                              # eval $(ax proxy off)
ps                              # ax proxy status
pn http://other:1080            # 自定义地址
```

## 系统管理

```bash
ax install                      # 一键安装
ax push / pull                  # 配置同步
ax info                         # 查看配置
ax completion bash/zsh/powershell # 安装补全
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

---

**返回** → [文档总览](./README.md)
