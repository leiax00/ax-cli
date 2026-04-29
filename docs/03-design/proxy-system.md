# 代理系统设计

## 概述

`ax proxy` 管理 HTTP/HTTPS 代理，为不同 shell 生成对应的环境变量设置命令。

## 配置

```yaml
# config.yaml
proxy:
  address: "http://127.0.0.1:7890"
  no_proxy: "localhost,127.0.0.1,10.0.0.0/8,172.16.0.0/12,192.168.0.0/16,*.local"
```

## 命令行为

### `ax proxy on`

根据当前 shell 类型输出代理设置命令。

- 传入 `addr` 时优先使用命令参数
- 未传入时读取 `config.proxy.address` 作为默认代理地址

- **bash/zsh**：
  ```bash
  export http_proxy="http://127.0.0.1:7890"
  export https_proxy="http://127.0.0.1:7890"
  export HTTP_PROXY="http://127.0.0.1:7890"
  export HTTPS_PROXY="http://127.0.0.1:7890"
  ```
- **PowerShell**：
  ```powershell
  $env:http_proxy="http://127.0.0.1:7890"
  $env:https_proxy="http://127.0.0.1:7890"
  ```
- **CMD**：
  ```cmd
  set http_proxy=http://127.0.0.1:7890
  set https_proxy=http://127.0.0.1:7890
  ```

### `ax proxy set-default <addr>`

将默认代理地址持久化写回 `config.yaml` 的 `proxy.address`。设置完成后可直接执行：

```bash
ax proxy on
```

### `ax proxy off`

输出清除代理的命令（unset 或设置为空）。

### `ax proxy status`

显示当前 shell 的代理状态，以及 `config.yaml` 中保存的默认代理地址。

## Shell 检测

通过环境变量 `SHELL` 和 `TERM_PROGRAM` 检测当前 shell 类型，自动选择输出格式。
