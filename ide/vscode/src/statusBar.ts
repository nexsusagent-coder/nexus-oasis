/**
 * Status Bar Manager
 */

import * as vscode from 'vscode';
import { SentientClient } from './client';

export class StatusBarManager {
    private statusItem: vscode.StatusBarItem;
    private client: SentientClient;

    constructor(client: SentientClient) {
        this.client = client;
        this.statusItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
        this.updateStatus();
        this.statusItem.show();

        // Update every 30 seconds
        setInterval(() => this.updateStatus(), 30000);
    }

    private async updateStatus() {
        const connected = this.client.isConnected();
        const model = this.client.getModel();

        if (connected) {
            this.statusItem.text = `$(hubot) ${model}`;
            this.statusItem.tooltip = 'SENTIENT OS: Connected';
            this.statusItem.command = 'sentient.models';
            this.statusItem.backgroundColor = undefined;
        } else {
            this.statusItem.text = '$(hubot) Disconnected';
            this.statusItem.tooltip = 'SENTIENT OS: Not connected. Click to configure.';
            this.statusItem.command = 'sentient.settings';
            this.statusItem.backgroundColor = new vscode.ThemeColor('statusBarItem.warningBackground');
        }
    }

    dispose() {
        this.statusItem.dispose();
    }
}
