#!/bin/bash
# proxy.sh - 代理启停管理
# 由 .zshrc 加载，提供 proxy_on / proxy_off / proxy_status 函数

PROXY_ADDR="${PROXY_ADDR:-http://vpn.yushe.ai:7890}"
NO_PROXY="${NO_PROXY:-localhost,127.0.0.1,10.0.0.0/8,172.16.0.0/12,192.168.0.0/16,*.local}"

proxy_on() {
  local addr="${1:-$PROXY_ADDR}"
  export http_proxy="$addr"
  export https_proxy="$PROXY_ADDR"
  export all_proxy="$PROXY_ADDR"
  export HTTP_PROXY="$PROXY_ADDR"
  export HTTPS_PROXY="$addr"
  export ALL_PROXY="$addr"
  export no_proxy="$NO_PROXY"
  export NO_PROXY="$NO_PROXY"
  echo "🟢 Proxy ON: $addr"
}

proxy_off() {
  unset http_proxy https_proxy all_proxy HTTP_PROXY HTTPS_PROXY ALL_PROXY no_proxy NO_PROXY
  echo "🔴 Proxy OFF"
}

proxy_status() {
  if [ -n "$http_proxy" ]; then
    echo "🟢 Proxy: $http_proxy"
    echo "   no_proxy: $no_proxy"
  else
    echo "🔴 Proxy: OFF"
  fi
}

# 短别名
alias pn='proxy_on'
alias pf='proxy_off'
alias ps='proxy_status'
