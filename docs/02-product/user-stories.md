# 用户故事与使用场景

## 场景 1：新机器环境搭建

> 作为开发者，我买了一台新电脑，希望一条命令完成开发环境搭建。

```bash
# 1. 安装 ax（或从 release 下载二进制）
# 2. 一键安装
ax install
```

期望结果：系统包、zsh、插件、fzf、starship、字体、配置文件全部就位。

## 场景 2：多机配置同步

> 作为开发者，我希望在多台机器间保持配置一致。

```bash
# 机器 A：首次初始化并推送
ax config init
ax config remote <git-repo-url>
ax push

# 机器 B：拉取配置
ax config init
ax config remote <git-repo-url>
ax pull
ax install
```

## 场景 3：自定义快捷命令

> 作为开发者，我经常执行一些复杂的组合命令，希望用简短别名替代。

```bash
ax add deploy "kubectl apply -k overlays/production" "部署到生产环境"
ax run deploy
```

## 场景 4：环境变量分组管理

> 作为开发者，我有不同项目需要不同的环境变量，希望按标签分组管理。

```bash
ax env add AWS_ACCESS_KEY_ID xxx --tag aws
ax env add DB_HOST localhost --tag local
ax env show --tag aws
ax env pause --tag aws    # 临时禁用
ax env resume --tag aws   # 恢复
```

## 场景 5：代理快速切换

> 作为开发者，我需要在不同网络环境间切换代理。

```bash
ax proxy set-default http://127.0.0.1:7890
ax proxy on     # 启用代理
ax proxy off    # 关闭代理
ax proxy status # 查看状态
```

## 场景 6：配置备份与迁移

> 作为开发者，我准备重装系统，需要备份和恢复配置。

```bash
# 备份
ax config export --output my-env.tar.gz

# 恢复
ax config init --force
ax config import my-env.tar.gz
```

## 场景 7：便携模式

> 作为开发者，我希望在 U 盘或特定目录下使用 ax，不依赖系统配置路径。

```bash
# 将 ax 二进制和 config/ 目录放在同一位置
# ax 会自动检测同级 config/ 目录作为配置根
./ax config path  # 显示便携模式路径
```
