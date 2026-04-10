/**
 * Models View Provider
 */

import * as vscode from 'vscode';
import { SentientClient, Model } from '../client';

export class ModelsViewProvider implements vscode.TreeDataProvider<ModelItem> {
    private _onDidChangeTreeData = new vscode.EventEmitter<ModelItem | undefined | null | void>();
    readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

    constructor(private readonly _client: SentientClient) {}

    refresh(): void {
        this._onDidChangeTreeData.fire();
    }

    getTreeItem(element: ModelItem): vscode.TreeItem {
        return element;
    }

    async getChildren(_element?: ModelItem): Promise<ModelItem[]> {
        const currentModel = this._client.getModel();
        const models = await this._client.getAvailableModels();

        return models.map(id => {
            const isCurrent = id === currentModel;
            return new ModelItem(
                id,
                isCurrent ? '✓ ' + id : id,
                isCurrent ? vscode.TreeItemCollapsibleState.None : vscode.TreeItemCollapsibleState.None,
                {
                    command: 'sentient.models',
                    title: 'Select Model',
                    arguments: []
                }
            );
        });
    }
}

class ModelItem extends vscode.TreeItem {
    constructor(
        public readonly id: string,
        public readonly label: string,
        public readonly collapsibleState: vscode.TreeItemCollapsibleState,
        public readonly command?: vscode.Command
    ) {
        super(label, collapsibleState);
        this.tooltip = id;
        this.contextValue = 'model';
    }
}
