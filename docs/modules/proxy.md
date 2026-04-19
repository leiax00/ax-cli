# 代理管理 (proxy.sh)

## 配置文件

`~/.ax/bash/proxy.sh`，由 `.zshrc` 加载。

## 用法

```bash
proxy_on                        # 使用默认代理
proxy_on http://other:1080      # 临时使用其他地址
proxy_off                       # 关闭代理
proxy_status                    # 查看状态
```

### 短别名

```bash
pn                              # proxy_on
pf                              # proxy_off
ps                              # proxy_status
```

### ax 调用

```bash
ax pn                           # 默认代理
ax pf                           # 关闭
ax ps                           # 状态
ax pn http://other:1080         # 自定义地址
```

## 默认配置

- 代理地址：`http://vpn.yushe.ai:7890`
- no_proxy：`localhost,127.0.0.1,10.0.0.0/8,172.16.0.0/12,192.168.0.0/16,*.local`

## 环境变量

`proxy_on` 设置以下变量：

```
http_proxy / HTTP_PROXY
https_proxy / HTTPS_PROXY
all_proxy / ALL_PROXY
no_proxy / NO_PROXY
```

`proxy_off` 清除所有上述变量。

## 自定义

编辑 `~/.ax/bash/proxy.sh`，修改：

- `PROXY_ADDR`：默认代理地址
- `NO_PROXY`：不代理的地址列表

修改后 `source ~/.zshrc` 生效。

---

**返回** → [模块列表](./README.md)
