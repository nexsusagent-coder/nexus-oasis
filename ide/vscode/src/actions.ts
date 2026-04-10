/**
 * Code Actions
 */

import * as vscode from 'vscode';
import { SentientClient } from './client';

export class CodeActions {
    constructor(private readonly _client: SentientClient) {}

    async explain(selection: { code: string; language: string }) {
        await vscode.window.withProgress(
            {
                location: vscode.ProgressLocation.Notification,
                title: 'SENTIENT: Explaining code...',
                cancellable: false
            },
            async () => {
                const result = await this._client.explain(selection.code, selection.language);
                await this.showResult('Code Explanation', result, selection.language);
            }
        );
    }

    async refactor(selection: { code: string; language: string }) {
        await vscode.window.withProgress(
            {
                location: vscode.ProgressLocation.Notification,
                title: 'SENTIENT: Refactoring code...',
                cancellable: false
            },
            async () => {
                const result = await this._client.refactor(selection.code, selection.language);
                await this.showResult('Refactored Code', result, selection.language, true);
            }
        );
    }

    async fix(selection: { code: string; language: string }) {
        await vscode.window.withProgress(
            {
                location: vscode.ProgressLocation.Notification,
                title: 'SENTIENT: Fixing code...',
                cancellable: false
            },
            async () => {
                const result = await this._client.fix(selection.code, selection.language);
                await this.showResult('Fixed Code', result, selection.language, true);
            }
        );
    }

    async generateTests(selection: { code: string; language: string }) {
        await vscode.window.withProgress(
            {
                location: vscode.ProgressLocation.Notification,
                title: 'SENTIENT: Generating tests...',
                cancellable: false
            },
            async () => {
                const testLang = this.getTestLanguage(selection.language);
                const result = await this._client.generateTests(selection.code, selection.language);
                await this.showResult('Generated Tests', result, testLang, true);
            }
        );
    }

    async generateDocs(selection: { code: string; language: string }) {
        await vscode.window.withProgress(
            {
                location: vscode.ProgressLocation.Notification,
                title: 'SENTIENT: Generating documentation...',
                cancellable: false
            },
            async () => {
                const result = await this._client.generateDocs(selection.code, selection.language);
                await this.showResult('Documentation', result, 'markdown');
            }
        );
    }

    async translate(selection: { code: string; language: string }, targetLang: string) {
        await vscode.window.withProgress(
            {
                location: vscode.ProgressLocation.Notification,
                title: `SENTIENT: Translating to ${targetLang}...`,
                cancellable: false
            },
            async () => {
                const result = await this._client.translate(
                    selection.code,
                    selection.language,
                    targetLang
                );
                await this.showResult(`Translated Code (${targetLang})`, result, targetLang.toLowerCase(), true);
            }
        );
    }

    async optimize(selection: { code: string; language: string }) {
        await vscode.window.withProgress(
            {
                location: vscode.ProgressLocation.Notification,
                title: 'SENTIENT: Optimizing code...',
                cancellable: false
            },
            async () => {
                const result = await this._client.optimize(selection.code, selection.language);
                await this.showResult('Optimized Code', result, selection.language, true);
            }
        );
    }

    async review(selection: { code: string; language: string }) {
        await vscode.window.withProgress(
            {
                location: vscode.ProgressLocation.Notification,
                title: 'SENTIENT: Reviewing code...',
                cancellable: false
            },
            async () => {
                const result = await this._client.review(selection.code, selection.language);
                await this.showResult('Code Review', result, 'markdown');
            }
        );
    }

    async generateCommitMessage() {
        await vscode.window.withProgress(
            {
                location: vscode.ProgressLocation.Notification,
                title: 'SENTIENT: Generating commit message...',
                cancellable: false
            },
            async () => {
                const result = await this._client.generateCommitMessage();
                
                // Copy to clipboard
                await vscode.env.clipboard.writeText(result);
                vscode.window.showInformationMessage('Commit message copied to clipboard!');
            }
        );
    }

    private async showResult(
        title: string,
        content: string,
        language: string = 'markdown',
        offerApply: boolean = false
    ) {
        const panel = vscode.window.createWebviewPanel(
            'sentientResult',
            title,
            vscode.ViewColumn.Beside,
            { enableScripts: true }
        );

        panel.webview.html = this.getResultHtml(title, content, language, offerApply);

        // Handle apply button
        panel.webview.onDidReceiveMessage(async (message) => {
            if (message.command === 'apply') {
                const editor = vscode.window.activeTextEditor;
                if (editor) {
                    // Extract code from markdown code blocks
                    const codeMatch = message.code.match(/```[\w]*\n([\s\S]*?)```/);
                    const code = codeMatch ? codeMatch[1] : message.code;
                    
                    await editor.edit(editBuilder => {
                        editBuilder.replace(editor.selection, code);
                    });
                    
                    vscode.window.showInformationMessage('Code applied!');
                    panel.dispose();
                }
            }
        });
    }

    private getResultHtml(title: string, content: string, language: string, offerApply: boolean): string {
        return `<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>${title}</title>
    <style>
        body {
            font-family: var(--vscode-editor-font-family);
            padding: 20px;
            color: var(--vscode-editor-foreground);
            background: var(--vscode-editor-background);
        }
        pre {
            background: var(--vscode-textCodeBlock-background);
            padding: 16px;
            border-radius: 8px;
            overflow-x: auto;
        }
        code {
            font-family: var(--vscode-editor-font-family);
        }
        button {
            background: var(--vscode-button-background);
            color: white;
            border: none;
            padding: 10px 20px;
            border-radius: 6px;
            cursor: pointer;
            margin-top: 16px;
        }
        button:hover {
            opacity: 0.9;
        }
    </style>
</head>
<body>
    <h2>${title}</h2>
    <pre><code class="language-${language}">${this.escapeHtml(content)}</code></pre>
    ${offerApply ? '<button onclick="applyCode()">Apply to Editor</button>' : ''}
    
    <script>
        const vscode = acquireVsCodeApi();
        
        function applyCode() {
            vscode.postMessage({
                command: 'apply',
                code: document.querySelector('code').textContent
            });
        }
    </script>
</body>
</html>`;
    }

    private escapeHtml(text: string): string {
        return text
            .replace(/&/g, '&amp;')
            .replace(/</g, '&lt;')
            .replace(/>/g, '&gt;')
            .replace(/"/g, '&quot;')
            .replace(/'/g, '&#039;');
    }

    private getTestLanguage(sourceLang: string): string {
        const testLangMap: Record<string, string> = {
            'typescript': 'typescript',
            'javascript': 'javascript',
            'python': 'python',
            'rust': 'rust',
            'go': 'go',
            'java': 'java',
            'csharp': 'csharp',
            'cpp': 'cpp',
            'ruby': 'ruby',
            'php': 'php'
        };
        return testLangMap[sourceLang] || 'plaintext';
    }
}
