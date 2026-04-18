#!/bin/bash
# lib/packages.sh - 系统包安装
# 依赖: lib/detect.sh (PKG_MANAGER, PKG_CHECK, PKG_UPDATE, PKG_INSTALL, PKG_LIST_FILE)

install_packages() {
  echo ""
  echo "📦 安装系统包..."

  local pkg_file="$DOTDIR/$PKG_LIST_FILE"
  if [ ! -f "$pkg_file" ]; then
    echo "  ⚠️  未找到包列表: $pkg_file"
    return 0
  fi

  $PKG_UPDATE 2>/dev/null || true

  local new_pkgs=()
  while IFS= read -r pkg; do
    # 跳过空行和注释
    [[ -z "$pkg" || "$pkg" == \#* ]] && continue
    $PKG_CHECK "$pkg" &>/dev/null || new_pkgs+=("$pkg")
  done < "$pkg_file"

  if [ ${#new_pkgs[@]} -gt 0 ]; then
    echo "  📥 新增包: ${new_pkgs[*]}"
    $PKG_INSTALL "${new_pkgs[@]}"
    echo "  ✅ 安装完成"
  else
    echo "  ⏭️  系统包齐全"
  fi
}
