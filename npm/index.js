/**
 * ═══════════════════════════════════════════════════════════════════════════════
 *  SENTIENT - AI Operating System
 * ═══════════════════════════════════════════════════════════════════════════════
 *
 *  Node.js wrapper for SENTIENT binary
 *
 *  Usage:
 *    const sentient = require('@sentient/ai');
 *    console.log(sentient.version);
 *    console.log(sentient.getBinaryPath());
 *
 *  CLI:
 *    sentient --version
 *    sentient repl
 *    sentient agent --goal "Build a REST API"
 *
 * ═══════════════════════════════════════════════════════════════════════════════
 */

const path = require('path');
const { spawn, spawnSync } = require('child_process');

const PLATFORM = process.platform;
const BINARY_NAME = PLATFORM === 'win32' ? 'sentient.exe' : 'sentient';
const BINARY_PATH = path.join(__dirname, 'bin', BINARY_NAME);

/**
 * Get the path to the SENTIENT binary
 * @returns {string} Path to binary
 */
function getBinaryPath() {
  return BINARY_PATH;
}

/**
 * Check if the binary exists
 * @returns {boolean}
 */
function isInstalled() {
  return require('fs').existsSync(BINARY_PATH);
}

/**
 * Get SENTIENT version
 * @returns {string|null} Version string or null if not installed
 */
function getVersion() {
  if (!isInstalled()) return null;
  
  try {
    const result = spawnSync(BINARY_PATH, ['--version'], {
      encoding: 'utf8',
      timeout: 5000
    });
    return result.stdout.trim();
  } catch {
    return null;
  }
}

/**
 * Execute SENTIENT command
 * @param {string[]} args - Command arguments
 * @param {Object} options - Spawn options
 * @returns {Promise<string>} Command output
 */
function exec(args, options = {}) {
  return new Promise((resolve, reject) => {
    if (!isInstalled()) {
      return reject(new Error('SENTIENT binary not found. Run: npm install @sentient/ai'));
    }
    
    const child = spawn(BINARY_PATH, args, {
      stdio: options.stdio || 'pipe',
      cwd: options.cwd || process.cwd(),
      env: { ...process.env, ...options.env }
    });
    
    let stdout = '';
    let stderr = '';
    
    if (child.stdout) {
      child.stdout.on('data', (data) => { stdout += data; });
    }
    if (child.stderr) {
      child.stderr.on('data', (data) => { stderr += data; });
    }
    
    child.on('error', reject);
    child.on('exit', (code) => {
      if (code === 0) {
        resolve(stdout);
      } else {
        const err = new Error(stderr || `SENTIENT exited with code ${code}`);
        err.code = code;
        err.stdout = stdout;
        err.stderr = stderr;
        reject(err);
      }
    });
  });
}

/**
 * Run SENTIENT REPL interactively
 * @param {Object} options - Options
 */
function repl(options = {}) {
  if (!isInstalled()) {
    console.error('❌  SENTIENT binary not found!');
    console.log('Run: npm install @sentient/ai');
    process.exit(1);
  }
  
  const args = ['repl'];
  if (options.swarm) args.push('--swarm');
  if (options.debug) args.push('--debug');
  
  spawn(BINARY_PATH, args, { stdio: 'inherit' });
}

/**
 * Run autonomous agent
 * @param {string} goal - Agent goal
 * @param {Object} options - Options
 * @returns {Promise<string>} Agent result
 */
async function agent(goal, options = {}) {
  const args = ['agent', '--goal', goal];
  if (options.model) args.push('--model', options.model);
  if (options.maxIterations) args.push('--max-iterations', String(options.maxIterations));
  
  return exec(args);
}

/**
 * Query LLM
 * @param {string} query - Query text
 * @param {Object} options - Options
 * @returns {Promise<string>} LLM response
 */
async function query(query, options = {}) {
  const args = ['llm', 'chat', '--model', options.model || 'qwen/qwen3-1.7b:free'];
  return exec(args, { input: query });
}

// Export
module.exports = {
  version: require('./package.json').version,
  getBinaryPath,
  isInstalled,
  getVersion,
  exec,
  repl,
  agent,
  query
};
