/**
 * Skills View Provider
 */

import * as vscode from 'vscode';
import { SentientClient, Skill } from '../client';

export class SkillsViewProvider implements vscode.TreeDataProvider<SkillItem> {
    private _onDidChangeTreeData = new vscode.EventEmitter<SkillItem | undefined | null | void>();
    readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

    constructor(private readonly _client: SentientClient) {}

    refresh(): void {
        this._onDidChangeTreeData.fire();
    }

    getTreeItem(element: SkillItem): vscode.TreeItem {
        return element;
    }

    async getChildren(element?: SkillItem): Promise<SkillItem[]> {
        if (!element) {
            // Root - show categories
            const categories = await this.getCategories();
            return categories.map(c => new SkillItem(
                c,
                c,
                vscode.TreeItemCollapsibleState.Collapsed,
                'category'
            ));
        } else if (element.contextValue === 'category') {
            // Show skills in category
            const skills = await this._client.getSkills();
            return skills
                .filter(s => s.category === element.id)
                .map(s => new SkillItem(
                    s.id,
                    s.name,
                    vscode.TreeItemCollapsibleState.None,
                    'skill',
                    s.description
                ));
        }
        return [];
    }

    private async getCategories(): Promise<string[]> {
        const skills = await this._client.getSkills();
        const categories = new Set(skills.map(s => s.category));
        return Array.from(categories).sort();
    }
}

class SkillItem extends vscode.TreeItem {
    constructor(
        public readonly id: string,
        public readonly label: string,
        public readonly collapsibleState: vscode.TreeItemCollapsibleState,
        public readonly contextValue: string,
        public readonly description?: string
    ) {
        super(label, collapsibleState);
        this.tooltip = description || label;
    }
}
