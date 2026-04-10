#!/usr/bin/env node
/**
 * SENTIENT AI - NPM Uninstall Script
 * 
 * Bu script npm uninstall sırasında çalışır ve
 * geçici dosyaları temizler.
 */

const fs = require('fs');
const path = require('path');
const os = require('os');

const PLATFORM = process.platform;
const BIN_DIR = path.join(__dirname, 'bin');

console.log('🗑️  SENTIENT AI uninstalling...');

// Binary dosyalarını temizle
function cleanBinaries() {
    if (!fs.existsSync(BIN_DIR)) {
        return;
    }

    const files = fs.readdirSync(BIN_DIR);
    let cleaned = 0;

    for (const file of files) {
        const filePath = path.join(BIN_DIR, file);
        try {
            fs.unlinkSync(filePath);
            cleaned++;
        } catch (err) {
            // Ignore errors during cleanup
        }
    }

    // Bin klasörünü sil
    try {
        fs.rmdirSync(BIN_DIR);
    } catch (err) {
        // Ignore if not empty
    }

    if (cleaned > 0) {
        console.log(`✅ Cleaned ${cleaned} binary file(s)`);
    }
}

// Config dosyalarını temizle
function cleanConfig() {
    const configDir = path.join(os.homedir(), '.sentient');
    
    if (fs.existsSync(configDir)) {
        console.log('📁 Config directory preserved at: ' + configDir);
        console.log('   To remove: rm -rf ~/.sentient');
    }
}

// Ana temizleme
try {
    cleanBinaries();
    cleanConfig();
    console.log('✅ Uninstall complete');
} catch (err) {
    console.error('⚠️  Uninstall warning:', err.message);
    // Exit successfully even if cleanup fails
}

process.exit(0);
