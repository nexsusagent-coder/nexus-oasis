#!/usr/bin/env node
/**
 * ═══════════════════════════════════════════════════════════════════════════════
 *  SENTIENT - CLI Wrapper
 * ═══════════════════════════════════════════════════════════════════════════════
 */

const { spawn } = require('child_process');
const path = require('path');

const PLATFORM = process.platform;
const binary = PLATFORM === 'win32' ? 'sentient.exe' : 'sentient';
const binaryPath = path.join(__dirname, '..', 'bin', binary);

// Check if binary exists
if (!require('fs').existsSync(binaryPath)) {
  console.error('❌  SENTIENT binary not found!');
  console.log('');
  console.log('Run: npm install @sentient/ai');
  console.log('Or download from: https://github.com/nexsusagent-coder/SENTIENT_CORE/releases');
  process.exit(1);
}

// Spawn the binary with all arguments
const args = process.argv.slice(2);
const child = spawn(binaryPath, args, {
  stdio: 'inherit',
  env: process.env
});

child.on('error', (err) => {
  console.error('❌  Failed to start SENTIENT:', err.message);
  process.exit(1);
});

child.on('exit', (code, signal) => {
  if (signal) {
    process.kill(process.pid, signal);
  } else {
    process.exit(code || 0);
  }
});
