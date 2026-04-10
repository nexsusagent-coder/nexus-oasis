/**
 * History View Provider
 */

import * as vscode from 'vscode';
import { SentientClient } from '../client';

interface HistoryItem {
    id: string;
    preview: string;
    timestamp: Date;
    messageCount: number;
}

export class HistoryViewProvider implements vscode.TreeDataProvider<HistoryTreeItem> {
    private _onDidChangeTreeData = new vscode.EventEmitter<HistoryTreeItem | undefined | null | void>();
    readonly onDidChangeTreeData = this._onDidChangeTreeData.event;
    private history: HistoryItem[] = [];

    constructor(
        private readonly _context: vscode.ExtensionContext,
        private readonly _client: SentientClient
    ) {
        this.loadHistory();
    }

    refresh(): void {
        this._onDidChangeTreeData.fire();
    }

    private async loadHistory() {
        // Load from extension context
        this.history = this._context.globalState.get<HistoryItem[]>('chatHistory', []);
    }

    async saveHistory(preview: string, messageCount: number) {
        const item: HistoryItem = {
            id: Date.now().toString(),
            preview: preview.substring(0, 50) + (preview.length > 50 ? '...' : ''),
            timestamp: new Date(),
            messageCount
        };

        this.history.unshift(item);
        
        // Keep only last 100 items
        if (this.history.length > 100) {
            this.history = this.history.slice(0, 100);
        }

        await this._context.globalState.update('chatHistory', this.history);
        this.refresh();
    }

    getTreeItem(element: HistoryTreeItem): vscode.TreeItem {
        return element;
    }

    getChildren(_element?: HistoryTreeItem): Promise<HistoryTreeItem[]> {
        return Promise.resolve(
            this.history.map(item => 
                new HistoryTreeItem(
                    item.id,
                    item.preview,
                    `${item.messageCount} messages • ${this.formatTime(item.timestamp)}`
                )
            )
        );
    }

    private formatTime(date: Date): string {
        const now = new Date();
        const diff = now.getTime() - new Date(date).getTime();
        
        if (diff < 60000) return 'Just now';
        if (diff < 3600000) return `${Math.floor(diff / 60000)}m ago`;
        if (diff < 86400000) return `${Math.floor(diff / 3600000)}h ago`;
        return `${Math.floor(diff / 86400000)}d ago`;
    }
}

class HistoryTreeItem extends vscode.TreeItem {
    constructor(
        public readonly id: string,
        public readonly label: string,
        public readonly description: string
    ) {
        super(label, vscode.TreeItemCollapsibleState.None);
        this.description = description;
        this.contextValue = 'historyItem';
        this.iconPath = new vscode.ThemeIcon('comment');
    }
}
