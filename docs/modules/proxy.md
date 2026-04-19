# 代理管理

## 用法

```bash
eval $(ax proxy on)                # 开启（使用默认代理）
eval $(ax proxy on http://other)   # 临时使用其他地址
eval $(ax proxy off)               # 关闭
ax proxy status                    # 查看状态
```

### zsh 快捷方式

`.zshrc` 中内置了别名（通过 `ax config init` 生成）：

```bash
pn                              # eval $(ax proxy on)
pf                              # eval $(ax proxy off)
ps                              # ax proxy status
```

## 默认配置

在 `config.yaml` 中配置：

```yaml
proxy:
  address: "http://vpn.yushe.ai:7890"
  no_proxy: "localhost,127.0.0.1,10.0.0.0/8,172.16.0.0/12,192.168.0.0/16,*.local"
```

## 设置的环境变量

`ax proxy on` 输出以下 shell 命令：

```
export http_proxy=...
export HTTP_PROXY=...
export https_proxy=...
export HTTPS_PROXY=...
export all_proxy=...
export ALL_PROXY=...
export no_proxy=...
export NO_PROXY=...
```

`ax proxy off` 输出 `unset` 命令清除所有变量。

> 因为子进程无法修改父进程的环境变量，所以需要 `eval $(...)` 包裹。

---

**返回** → [模块列表](./README.md)
