<#
.SYNOPSIS
    ax-cli Windows 安装脚本
.DESCRIPTION
    自动下载 ax 二进制到 %LOCALAPPDATA%\ax-cli\bin 并加入 PATH
.EXAMPLE
    irm https://anyhub.yushe.ai/leiax00/ax-system-basic/raw/branch/main/install-ax.ps1 | iex
#>

param(
    [string]$Version = "latest"
)

$ErrorActionPreference = "Stop"
$Repo = "leiax00/ax-system-basic"
$GiteaBase = "https://anyhub.yushe.ai"
$InstallDir = "$env:LOCALAPPDATA\ax-cli\bin"

Write-Host ""
Write-Host "  ===================================="
Write-Host "  |       ax-cli 安装程序            |"
Write-Host "  ===================================="
Write-Host ""

# 检测架构
$arch = switch ($env:PROCESSOR_ARCHITECTURE) {
    "AMD64"   { "x86_64" }
    "ARM64"   { "aarch64" }
    default   { Write-Error "不支持的架构: $_"; exit 1 }
}

$platform = "windows-$arch"
$filename = "ax-windows-$arch.exe"
Write-Host "[INFO] 平台: $platform"

# 确定下载 URL
if ($Version -eq "latest") {
    $url = "$GiteaBase/$Repo/releases/latest/download/$filename"
} else {
    $url = "$GiteaBase/$Repo/releases/download/$Version/$filename"
}

# 下载
Write-Host "[INFO] 下载: $url"
$tmpFile = Join-Path $env:TEMP "ax-install.exe"

try {
    Invoke-WebRequest -Uri $url -OutFile $tmpFile -UseBasicParsing
} catch {
    # 备用 GitHub
    $ghUrl = "https://github.com/$Repo/releases/$Version/download/$filename"
    Write-Host "[WARN] Gitea 下载失败，尝试 GitHub..."
    try {
        Invoke-WebRequest -Uri $ghUrl -OutFile $tmpFile -UseBasicParsing
    } catch {
        Remove-Item $tmpFile -ErrorAction SilentlyContinue
        Write-Error "下载失败: $_"
        exit 1
    }
}

# 安装
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

$dest = Join-Path $InstallDir "ax.exe"
if (Test-Path $dest) {
    $backup = "$dest.bak.$(Get-Date -Format 'yyyyMMddHHmmss')"
    Move-Item $dest $backup
    Write-Host "[INFO] 已备份: $backup"
}

Move-Item $tmpFile $dest -Force
Write-Host "[INFO] 已安装: $dest"

# 添加到用户 PATH（仅当前用户）
$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($userPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$userPath;$InstallDir", "User")
    $env:PATH += ";$InstallDir"
    Write-Host "[INFO] 已添加到用户 PATH"
} else {
    Write-Host "[INFO] PATH 中已存在"
}

# 验证
Write-Host ""
try {
    $installed = & $dest --version 2>$null
    Write-Host "[INFO] 安装成功! ax $installed" -ForegroundColor Green
} catch {
    Write-Host "[INFO] 安装完成!" -ForegroundColor Green
}

Write-Host ""
Write-Host "  下一步:"
Write-Host "    ax config init    # 初始化配置"
Write-Host "    ax install        # 安装工具"
Write-Host "    (请重启终端使 PATH 生效)"
Write-Host ""
