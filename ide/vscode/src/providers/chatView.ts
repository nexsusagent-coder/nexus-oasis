/**
 * Chat View Provider
 */

import * as vscode from 'vscode';
import { SentientClient, Message } from '../client';

export class ChatViewProvider implements vscode.WebviewViewProvider {
    private _view?: vscode.WebviewView;
    private messages: Message[] = [];

    constructor(
        private readonly _extensionUri: vscode.Uri,
        private readonly _client: SentientClient
    ) {}

    public resolveWebviewView(
        webviewView: vscode.WebviewView,
        _context: vscode.WebviewViewResolveContext,
        _token: vscode.CancellationToken
    ) {
        this._view = webviewView;

        webviewView.webview.options = {
            enableScripts: true,
            localResourceRoots: [this._extensionUri]
        };

        webviewView.webview.html = this._getHtmlForWebview(webviewView.webview);

        // Handle messages from webview
        webviewView.webview.onDidReceiveMessage(async (data) => {
            switch (data.type) {
                case 'sendMessage':
                    await this.handleUserMessage(data.message);
                    break;
                case 'clearHistory':
                    this.messages = [];
                    this._updateChat();
                    break;
            }
        });
    }

    private async handleUserMessage(content: string) {
        if (!content.trim()) {
            return;
        }

        // Add user message
        const userMessage: Message = {
            role: 'user',
            content,
            timestamp: new Date()
        };
        this.messages.push(userMessage);
        this._updateChat();

        // Show loading
        this._view?.webview.postMessage({ type: 'loading', value: true });

        try {
            const response = await this._client.chat(this.messages);
            
            const assistantMessage: Message = {
                role: 'assistant',
                content: response.message,
                timestamp: new Date()
            };
            this.messages.push(assistantMessage);
        } catch (error: any) {
            const errorMessage: Message = {
                role: 'assistant',
                content: `❌ Error: ${error.message}`,
                timestamp: new Date()
            };
            this.messages.push(errorMessage);
        }

        this._updateChat();
        this._view?.webview.postMessage({ type: 'loading', value: false });
    }

    private _updateChat() {
        this._view?.webview.postMessage({
            type: 'updateMessages',
            messages: this.messages.map(m => ({
                role: m.role,
                content: m.content,
                timestamp: m.timestamp.toISOString()
            }))
        });
    }

    public clearHistory() {
        this.messages = [];
        this._updateChat();
    }

    public addContextMessage(content: string, role: 'user' | 'assistant' = 'user') {
        this.messages.push({
            role,
            content,
            timestamp: new Date()
        });
        this._updateChat();
    }

    private _getHtmlForWebview(webview: vscode.Webview): string {
        return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SENTIENT Chat</title>
    <style>
        :root {
            --bg-primary: var(--vscode-editor-background);
            --bg-secondary: var(--vscode-input-background);
            --text-primary: var(--vscode-editor-foreground);
            --text-secondary: var(--vscode-descriptionForeground);
            --border: var(--vscode-input-border);
            --accent: var(--vscode-button-background);
        }

        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }

        body {
            font-family: var(--vscode-font-family);
            background: var(--bg-primary);
            color: var(--text-primary);
            height: 100vh;
            display: flex;
            flex-direction: column;
        }

        .chat-container {
            flex: 1;
            overflow-y: auto;
            padding: 12px;
        }

        .message {
            margin-bottom: 16px;
            padding: 12px;
            border-radius: 8px;
            max-width: 95%;
        }

        .message.user {
            background: var(--accent);
            margin-left: auto;
        }

        .message.assistant {
            background: var(--bg-secondary);
        }

        .message-header {
            font-size: 11px;
            font-weight: 600;
            text-transform: uppercase;
            margin-bottom: 6px;
            opacity: 0.7;
        }

        .message-content {
            font-size: 13px;
            line-height: 1.5;
            white-space: pre-wrap;
            word-wrap: break-word;
        }

        .message-content code {
            background: rgba(0,0,0,0.2);
            padding: 2px 6px;
            border-radius: 4px;
            font-family: var(--vscode-editor-font-family);
        }

        .message-content pre {
            background: rgba(0,0,0,0.2);
            padding: 12px;
            border-radius: 6px;
            overflow-x: auto;
            margin: 8px 0;
        }

        .input-container {
            padding: 12px;
            border-top: 1px solid var(--border);
            display: flex;
            gap: 8px;
        }

        #messageInput {
            flex: 1;
            background: var(--bg-secondary);
            border: 1px solid var(--border);
            border-radius: 6px;
            padding: 10px 12px;
            color: var(--text-primary);
            font-size: 13px;
            resize: none;
            min-height: 40px;
            max-height: 120px;
        }

        #messageInput:focus {
            outline: none;
            border-color: var(--accent);
        }

        #sendButton {
            background: var(--accent);
            border: none;
            border-radius: 6px;
            padding: 10px 16px;
            color: white;
            cursor: pointer;
            font-weight: 500;
        }

        #sendButton:hover {
            opacity: 0.9;
        }

        #sendButton:disabled {
            opacity: 0.5;
            cursor: not-allowed;
        }

        .loading {
            text-align: center;
            padding: 20px;
            color: var(--text-secondary);
        }

        .loading::after {
            content: '...';
            animation: dots 1.5s infinite;
        }

        @keyframes dots {
            0%, 20% { content: '.'; }
            40% { content: '..'; }
            60%, 100% { content: '...'; }
        }

        .empty-state {
            text-align: center;
            padding: 40px 20px;
            color: var(--text-secondary);
        }

        .empty-state h3 {
            margin-bottom: 8px;
            font-size: 14px;
        }

        .empty-state p {
            font-size: 12px;
            opacity: 0.7;
        }
    </style>
</head>
<body>
    <div id="chatContainer" class="chat-container">
        <div class="empty-state">
            <h3>🤖 SENTIENT OS</h3>
            <p>Ask anything about your code</p>
        </div>
    </div>
    
    <div id="loading" class="loading" style="display: none;">Thinking</div>
    
    <div class="input-container">
        <textarea id="messageInput" placeholder="Ask SENTIENT..." rows="1"></textarea>
        <button id="sendButton">Send</button>
    </div>

    <script>
        const vscode = acquireVsCodeApi();
        const chatContainer = document.getElementById('chatContainer');
        const messageInput = document.getElementById('messageInput');
        const sendButton = document.getElementById('sendButton');
        const loadingEl = document.getElementById('loading');
        let messages = [];

        function renderMessages() {
            if (messages.length === 0) {
                chatContainer.innerHTML = '<div class="empty-state"><h3>🤖 SENTIENT OS</h3><p>Ask anything about your code</p></div>';
                return;
            }

            chatContainer.innerHTML = messages.map(m => {
                const time = new Date(m.timestamp).toLocaleTimeString();
                return '<div class="message ' + m.role + '">' +
                    '<div class="message-header">' + m.role + ' • ' + time + '</div>' +
                    '<div class="message-content">' + formatContent(m.content) + '</div>' +
                '</div>';
            }).join('');
            
            chatContainer.scrollTop = chatContainer.scrollHeight;
        }

        function formatContent(content) {
            // Simple markdown-like formatting
            return content
                .replace(/\`\`\`(\\w*)\\n([\\s\\S]*?)\`\`\`/g, '<pre><code>$2</code></pre>')
                .replace(/\`([^\`]+)\`/g, '<code>$1</code>')
                .replace(/\\*\\*([^*]+)\\*\\*/g, '<strong>$1</strong>')
                .replace(/\\*([^*]+)\\*/g, '<em>$1</em>');
        }

        function sendMessage() {
            const content = messageInput.value.trim();
            if (!content) return;
            
            vscode.postMessage({ type: 'sendMessage', message: content });
            messageInput.value = '';
        }

        sendButton.addEventListener('click', sendMessage);
        messageInput.addEventListener('keydown', (e) => {
            if (e.key === 'Enter' && !e.shiftKey) {
                e.preventDefault();
                sendMessage();
            }
        });

        window.addEventListener('message', event => {
            const data = event.data;
            switch (data.type) {
                case 'updateMessages':
                    messages = data.messages;
                    renderMessages();
                    break;
                case 'loading':
                    loadingEl.style.display = data.value ? 'block' : 'none';
                    sendButton.disabled = data.value;
                    break;
            }
        });
    </script>
</body>
</html>`;
    }
}
