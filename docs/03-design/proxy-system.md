# 代理系统设计

## 概述

`ax proxy` 管理 HTTP/HTTPS 代理，为不同 shell 生成对应的环境变量设置命令。

## 配置

```yaml
# config.yaml
proxy:
  host: "127.0.0.1"
  port: "7890"
```

## 命令行为

### `ax proxy on`

根据当前 shell 类型输出代理设置命令：

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

### `ax proxy off`

输出清除代理的命令（unset 或设置为空）。

### `ax proxy status`

显示当前代理配置（host:port）和状态。

## Shell 检测

通过环境变量 `SHELL` 和 `TERM_PROGRAM` 检测当前 shell 类型，自动选择输出格式。
