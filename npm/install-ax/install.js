#!/usr/bin/env node
'use strict';

const { createReadStream, createWriteStream, chmodSync, mkdirSync, existsSync } = require('fs');
const { join, dirname } = require('path');
const { get } = require('https');
const { pipeline } = require('stream/promises');
const { arch, platform, homedir } = require('os');
const { execSync } = require('child_process');

const REPO = 'leiax00/ax-system-basic';
const GITEA_BASE = 'https://anyhub.yushe.ai';

// 平台映射
function getPlatform() {
    const a = arch();
    const p = platform();
    const archMap = { x64: 'x86_64', arm64: 'aarch64' };
    const platMap = { darwin: 'macos', linux: 'linux', win32: 'windows' };
    return `${platMap[p]}-${archMap[a]}`;
}

function getBinaryName() {
    return platform() === 'win32' ? 'ax.exe' : 'ax';
}

function getInstallDir() {
    if (platform() === 'win32') {
        return join(process.env.LOCALAPPDATA || homedir(), 'ax-cli', 'bin');
    }
    return join(homedir(), '.local', 'bin');
}

function getFilename(platform) {
    return `ax-${platform}${platform.startsWith('windows') ? '.exe' : ''}`;
}

function getDownloadUrl(version) {
    const plat = getPlatform();
    const fn = getFilename(plat);
    if (version === 'latest') {
        return `${GITEA_BASE}/${REPO}/releases/latest/download/${fn}`;
    }
    return `${GITEA_BASE}/${REPO}/releases/download/${version}/${fn}`;
}

function download(url, dest) {
    return new Promise((resolve, reject) => {
        const file = createWriteStream(dest);
        function tryDownload(targetUrl) {
            get(targetUrl, (res) => {
                if (res.statusCode === 302 || res.statusCode === 301) {
                    tryDownload(res.headers.location);
                    return;
                }
                if (res.statusCode !== 200) {
                    reject(new Error(`HTTP ${res.statusCode}: ${url}`));
                    return;
                }
                pipeline(res, file).then(resolve).catch(reject);
            }).on('error', reject);
        }
        tryDownload(url);
    });
}

async function main() {
    const version = process.env.AX_VERSION || 'latest';
    const plat = getPlatform();
    const binName = getBinaryName();
    const installDir = getInstallDir();
    const dest = join(installDir, binName);

    console.log(`[ax-cli] 平台: ${plat}`);
    console.log(`[ax-cli] 安装到: ${installDir}`);

    // 创建目录
    mkdirSync(installDir, { recursive: true });

    // 下载
    const url = getDownloadUrl(version);
    console.log(`[ax-cli] 下载: ${url}`);

    const tmpFile = dest + '.tmp';
    try {
        await download(url, tmpFile);
    } catch {
        // 备用 GitHub
        const fn = getFilename(plat);
        const ghUrl = `https://github.com/${REPO}/releases/${version}/download/${fn}`;
        console.log(`[ax-cli] 备用下载: ${ghUrl}`);
        await download(ghUrl, tmpFile);
    }

    // 替换
    if (existsSync(dest)) {
        const backup = `${dest}.bak`;
        require('fs').renameSync(dest, backup);
    }

    require('fs').renameSync(tmpFile, dest);

    // chmod (非 Windows)
    if (platform() !== 'win32') {
        chmodSync(dest, 0o755);
    }

    console.log(`[ax-cli] ✅ 安装成功: ${dest}`);
}

main().catch(err => {
    console.error(`[ax-cli] ❌ 安装失败: ${err.message}`);
    process.exit(1);
});
