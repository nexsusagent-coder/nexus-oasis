/**
 * SENTIENT OS VS Code Extension
 * AI-powered coding assistant with SENTIENT OS integration
 */

import * as vscode from 'vscode';
import { SentientClient } from './client';
import { ChatViewProvider } from './providers/chatView';
import { ModelsViewProvider } from './providers/modelsView';
import { SkillsViewProvider } from './providers/skillsView';
import { HistoryViewProvider } from './providers/historyView';
import { CodeActions } from './actions';
import { StatusBarManager } from './statusBar';

let client: SentientClient;
let chatProvider: ChatViewProvider;
let statusBar: StatusBarManager;

export async function activate(context: vscode.ExtensionContext) {
    console.log('SENTIENT OS extension is activating...');

    // Initialize client
    const config = vscode.workspace.getConfiguration('sentient');
    const apiUrl = config.get<string>('apiUrl', 'http://localhost:8080');
    const model = config.get<string>('model', 'gpt-4-turbo');
    
    client = new SentientClient(apiUrl, model);

    // Initialize status bar
    statusBar = new StatusBarManager(client);
    context.subscriptions.push(statusBar);

    // Initialize providers
    chatProvider = new ChatViewProvider(context.extensionUri, client);
    const modelsProvider = new ModelsViewProvider(client);
    const skillsProvider = new SkillsViewProvider(client);
    const historyProvider = new HistoryViewProvider(context, client);

    // Register views
    context.subscriptions.push(
        vscode.window.registerWebviewViewProvider('sentient.chatView', chatProvider)
    );
    context.subscriptions.push(
        vscode.window.registerTreeDataProvider('sentient.modelsView', modelsProvider)
    );
    context.subscriptions.push(
        vscode.window.registerTreeDataProvider('sentient.skillsView', skillsProvider)
    );
    context.subscriptions.push(
        vscode.window.registerTreeDataProvider('sentient.historyView', historyProvider)
    );

    // Initialize code actions
    const codeActions = new CodeActions(client);

    // Register commands
    registerCommands(context, client, chatProvider, codeActions, modelsProvider);

    // Check API connection
    const connected = await client.checkConnection();
    if (connected) {
        vscode.window.showInformationMessage('✅ SENTIENT OS connected');
    } else {
        vscode.window.showWarningMessage('⚠️ SENTIENT OS not connected. Check API settings.');
    }

    console.log('SENTIENT OS extension activated successfully');
}

function registerCommands(
    context: vscode.ExtensionContext,
    client: SentientClient,
    chatProvider: ChatViewProvider,
    codeActions: CodeActions,
    modelsProvider: ModelsViewProvider
) {
    // Chat command
    context.subscriptions.push(
        vscode.commands.registerCommand('sentient.chat', () => {
            vscode.commands.executeCommand('workbench.view.extension.sentient-sidebar');
        })
    );

    // Explain code
    context.subscriptions.push(
        vscode.commands.registerCommand('sentient.explain', async () => {
            const selection = getSelectedCode();
            if (selection) {
                await codeActions.explain(selection);
            }
        })
    );

    // Refactor code
    context.subscriptions.push(
        vscode.commands.registerCommand('sentient.refactor', async () => {
            const selection = getSelectedCode();
            if (selection) {
                await codeActions.refactor(selection);
            }
        })
    );

    // Fix code
    context.subscriptions.push(
        vscode.commands.registerCommand('sentient.fix', async () => {
            const selection = getSelectedCode();
            if (selection) {
                await codeActions.fix(selection);
            }
        })
    );

    // Generate tests
    context.subscriptions.push(
        vscode.commands.registerCommand('sentient.test', async () => {
            const selection = getSelectedCode();
            if (selection) {
                await codeActions.generateTests(selection);
            }
        })
    );

    // Generate documentation
    context.subscriptions.push(
        vscode.commands.registerCommand('sentient.document', async () => {
            const selection = getSelectedCode();
            if (selection) {
                await codeActions.generateDocs(selection);
            }
        })
    );

    // Translate code
    context.subscriptions.push(
        vscode.commands.registerCommand('sentient.translate', async () => {
            const selection = getSelectedCode();
            if (selection) {
                const languages = ['Python', 'JavaScript', 'TypeScript', 'Rust', 'Go', 'Java', 'C++', 'C#'];
                const target = await vscode.window.showQuickPick(languages, {
                    placeHolder: 'Select target language'
                });
                if (target) {
                    await codeActions.translate(selection, target);
                }
            }
        })
    );

    // Optimize code
    context.subscriptions.push(
        vscode.commands.registerCommand('sentient.optimize', async () => {
            const selection = getSelectedCode();
            if (selection) {
                await codeActions.optimize(selection);
            }
        })
    );

    // Code review
    context.subscriptions.push(
        vscode.commands.registerCommand('sentient.review', async () => {
            const selection = getSelectedCode();
            if (selection) {
                await codeActions.review(selection);
            }
        })
    );

    // Generate commit message
    context.subscriptions.push(
        vscode.commands.registerCommand('sentient.commit', async () => {
            await codeActions.generateCommitMessage();
        })
    );

    // Open settings
    context.subscriptions.push(
        vscode.commands.registerCommand('sentient.settings', () => {
            vscode.commands.executeCommand('workbench.action.openSettings', 'sentient');
        })
    );

    // Select model
    context.subscriptions.push(
        vscode.commands.registerCommand('sentient.models', async () => {
            const models = await client.getAvailableModels();
            const selected = await vscode.window.showQuickPick(models, {
                placeHolder: 'Select LLM model'
            });
            if (selected) {
                await client.setModel(selected);
                vscode.window.showInformationMessage(`Model set to: ${selected}`);
                modelsProvider.refresh();
            }
        })
    );

    // Browse skills
    context.subscriptions.push(
        vscode.commands.registerCommand('sentient.skills', async () => {
            vscode.commands.executeCommand('workbench.view.extension.sentient-sidebar');
            vscode.commands.executeCommand('sentient.skillsView.focus');
        })
    );

    // Clear history
    context.subscriptions.push(
        vscode.commands.registerCommand('sentient.clearHistory', () => {
            chatProvider.clearHistory();
            vscode.window.showInformationMessage('Chat history cleared');
        })
    );
}

function getSelectedCode(): { code: string; language: string } | undefined {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showWarningMessage('No active editor');
        return undefined;
    }

    const selection = editor.selection;
    if (selection.isEmpty) {
        vscode.window.showWarningMessage('No code selected');
        return undefined;
    }

    const code = editor.document.getText(selection);
    const language = editor.document.languageId;

    return { code, language };
}

export function deactivate() {
    console.log('SENTIENT OS extension deactivated');
    if (client) {
        client.disconnect();
    }
}
