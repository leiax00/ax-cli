#!/usr/bin/env node
'use strict';

// npm uninstall 时清理二进制
const { existsSync, unlinkSync } = require('fs');
const { join, homedir } = require('path');
const { platform } = require('os');

const binName = platform() === 'win32' ? 'ax.exe' : 'ax';
let installDir;

if (platform() === 'win32') {
    installDir = join(process.env.LOCALAPPDATA || homedir(), 'ax-cli', 'bin');
} else {
    installDir = join(homedir(), '.local', 'bin');
}

const dest = join(installDir, binName);
if (existsSync(dest)) {
    unlinkSync(dest);
    console.log(`[ax-cli] 已移除: ${dest}`);
}
