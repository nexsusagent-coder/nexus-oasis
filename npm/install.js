#!/usr/bin/env node
/**
 * ═══════════════════════════════════════════════════════════════════════════════
 *  SENTIENT - Binary Installer
 * ═══════════════════════════════════════════════════════════════════════════════
 */

const fs = require('fs');
const path = require('path');
const https = require('https');
const { execSync, spawn } = require('child_process');

const REPO = 'nexsusagent-coder/SENTIENT_CORE';
const PLATFORM = process.platform;
const ARCH = process.arch;

// Platform mapping
const PLATFORM_MAP = {
  'linux-x64': 'sentient-linux-x86_64.tar.gz',
  'linux-arm64': 'sentient-linux-arm64.tar.gz',
  'darwin-x64': 'sentient-macos-x86_64.tar.gz',
  'darwin-arm64': 'sentient-macos-arm64.tar.gz',
  'win32-x64': 'sentient-windows-x86_64.zip'
};

async function download(url, dest) {
  return new Promise((resolve, reject) => {
    console.log('📥  Downloading...');
    console.log(`    ${url}`);
    
    const file = fs.createWriteStream(dest);
    
    const follow = (url) => {
      https.get(url, (res) => {
        if (res.statusCode === 302 || res.statusCode === 301) {
          follow(res.headers.location);
        } else if (res.statusCode === 200) {
          res.pipe(file);
          file.on('finish', () => {
            file.close();
            resolve();
          });
        } else {
          reject(new Error(`HTTP ${res.statusCode}`));
        }
      }).on('error', reject);
    };
    
    follow(url);
  });
}

async function extract(file, dest) {
  console.log('📦  Extracting...');
  
  if (file.endsWith('.zip')) {
    // Windows - use PowerShell
    execSync(`powershell -command "Expand-Archive -Path '${file}' -DestinationPath '${dest}' -Force"`);
  } else {
    // Unix - use tar
    execSync(`tar -xzf '${file}' -C '${dest}'`);
  }
}

async function main() {
  const binDir = path.join(__dirname, 'bin');
  fs.mkdirSync(binDir, { recursive: true });
  
  const key = `${PLATFORM}-${ARCH}`;
  const binaryFile = PLATFORM_MAP[key];
  
  if (!binaryFile) {
    console.error(`❌  Unsupported platform: ${key}`);
    console.log('');
    console.log('Supported platforms:');
    console.log('  - linux-x64 (Linux x86_64)');
    console.log('  - linux-arm64 (Linux ARM64)');
    console.log('  - darwin-x64 (macOS Intel)');
    console.log('  - darwin-arm64 (macOS Apple Silicon)');
    console.log('  - win32-x64 (Windows x86_64)');
    process.exit(1);
  }
  
  // Check if binary already exists
  const binaryName = PLATFORM === 'win32' ? 'sentient.exe' : 'sentient';
  const binaryPath = path.join(binDir, binaryName);
  
  if (fs.existsSync(binaryPath)) {
    console.log('✅  Binary already installed');
    return;
  }
  
  // Get version from package.json
  const pkg = require('./package.json');
  const version = pkg.version;
  
  // Download
  const url = `https://github.com/${REPO}/releases/download/v${version}/${binaryFile}`;
  const tempFile = path.join(binDir, binaryFile);
  
  try {
    await download(url, tempFile);
    await extract(tempFile, binDir);
    
    // Cleanup
    fs.unlinkSync(tempFile);
    
    // Make executable (Unix)
    if (PLATFORM !== 'win32') {
      fs.chmodSync(binaryPath, 0o755);
    }
    
    console.log('');
    console.log('✅  Installation complete!');
    console.log('');
    console.log('Run: sentient --version');
    
  } catch (err) {
    console.error('❌  Download failed:', err.message);
    console.log('');
    console.log('💡  Please install manually from:');
    console.log(`    https://github.com/${REPO}/releases`);
    
    // Create placeholder
    fs.writeFileSync(binaryPath, `#!/bin/sh\necho "Please download binary from https://github.com/${REPO}/releases"`);
    if (PLATFORM !== 'win32') {
      fs.chmodSync(binaryPath, 0o755);
    }
  }
}

main().catch(console.error);
