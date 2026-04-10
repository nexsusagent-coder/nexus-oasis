/**
 * SENTIENT OS API Client
 */

import axios, { AxiosInstance } from 'axios';
import WebSocket from 'ws';
import { EventEmitter } from 'events';

export interface Message {
    role: 'user' | 'assistant' | 'system';
    content: string;
    timestamp: Date;
}

export interface Model {
    id: string;
    name: string;
    provider: string;
    contextLength: number;
    pricing?: {
        input: number;
        output: number;
    };
}

export interface Skill {
    id: string;
    name: string;
    description: string;
    category: string;
    version: string;
}

export interface ChatResponse {
    message: string;
    model: string;
    usage?: {
        promptTokens: number;
        completionTokens: number;
        totalTokens: number;
    };
}

export class SentientClient extends EventEmitter {
    private apiUrl: string;
    private model: string;
    private httpClient: AxiosInstance;
    private ws: WebSocket | null = null;
    private connected: boolean = false;

    constructor(apiUrl: string, model: string) {
        super();
        this.apiUrl = apiUrl;
        this.model = model;
        this.httpClient = axios.create({
            baseURL: apiUrl,
            timeout: 120000,
            headers: {
                'Content-Type': 'application/json'
            }
        });
    }

    async checkConnection(): Promise<boolean> {
        try {
            const response = await this.httpClient.get('/health');
            this.connected = response.status === 200;
            return this.connected;
        } catch {
            this.connected = false;
            return false;
        }
    }

    async chat(messages: Message[], stream: boolean = true): Promise<ChatResponse> {
        try {
            const response = await this.httpClient.post('/api/chat', {
                messages: messages.map(m => ({
                    role: m.role,
                    content: m.content
                })),
                model: this.model,
                stream: false,
                max_tokens: 4096
            });

            return {
                message: response.data.choices[0].message.content,
                model: response.data.model,
                usage: response.data.usage ? {
                    promptTokens: response.data.usage.prompt_tokens,
                    completionTokens: response.data.usage.completion_tokens,
                    totalTokens: response.data.usage.total_tokens
                } : undefined
            };
        } catch (error: any) {
            throw new Error(`Chat failed: ${error.message}`);
        }
    }

    async *chatStream(messages: Message[]): AsyncGenerator<string> {
        // WebSocket streaming implementation
        return new Promise((resolve, reject) => {
            const wsUrl = this.apiUrl.replace('http', 'ws') + '/ws/chat';
            this.ws = new WebSocket(wsUrl);

            this.ws.on('open', () => {
                this.ws?.send(JSON.stringify({
                    messages,
                    model: this.model,
                    stream: true
                }));
            });

            this.ws.on('message', (data: Buffer) => {
                const chunk = data.toString();
                const parsed = JSON.parse(chunk);
                if (parsed.done) {
                    this.ws?.close();
                    resolve();
                } else {
                    this.emit('chunk', parsed.content);
                }
            });

            this.ws.on('error', (error) => {
                reject(error);
            });
        });
    }

    async explain(code: string, language: string): Promise<string> {
        const response = await this.chat([
            { role: 'system', content: 'You are a code explanation expert. Explain the following code clearly and concisely.', timestamp: new Date() },
            { role: 'user', content: `Explain this ${language} code:\n\n\`\`\`${language}\n${code}\n\`\`\``, timestamp: new Date() }
        ]);
        return response.message;
    }

    async refactor(code: string, language: string): Promise<string> {
        const response = await this.chat([
            { role: 'system', content: 'You are a code refactoring expert. Improve the code quality while maintaining functionality.', timestamp: new Date() },
            { role: 'user', content: `Refactor this ${language} code for better readability and performance:\n\n\`\`\`${language}\n${code}\n\`\`\``, timestamp: new Date() }
        ]);
        return response.message;
    }

    async fix(code: string, language: string): Promise<string> {
        const response = await this.chat([
            { role: 'system', content: 'You are a code debugging expert. Find and fix issues in the code.', timestamp: new Date() },
            { role: 'user', content: `Find and fix bugs in this ${language} code:\n\n\`\`\`${language}\n${code}\n\`\`\``, timestamp: new Date() }
        ]);
        return response.message;
    }

    async generateTests(code: string, language: string): Promise<string> {
        const response = await this.chat([
            { role: 'system', content: 'You are a test generation expert. Write comprehensive unit tests.', timestamp: new Date() },
            { role: 'user', content: `Generate unit tests for this ${language} code:\n\n\`\`\`${language}\n${code}\n\`\`\``, timestamp: new Date() }
        ]);
        return response.message;
    }

    async generateDocs(code: string, language: string): Promise<string> {
        const response = await this.chat([
            { role: 'system', content: 'You are a documentation expert. Generate clear and comprehensive documentation.', timestamp: new Date() },
            { role: 'user', content: `Generate documentation for this ${language} code:\n\n\`\`\`${language}\n${code}\n\`\`\``, timestamp: new Date() }
        ]);
        return response.message;
    }

    async translate(code: string, sourceLang: string, targetLang: string): Promise<string> {
        const response = await this.chat([
            { role: 'system', content: `You are a code translation expert. Translate code from ${sourceLang} to ${targetLang}.`, timestamp: new Date() },
            { role: 'user', content: `Translate this ${sourceLang} code to ${targetLang}:\n\n\`\`\`${sourceLang}\n${code}\n\`\`\``, timestamp: new Date() }
        ]);
        return response.message;
    }

    async optimize(code: string, language: string): Promise<string> {
        const response = await this.chat([
            { role: 'system', content: 'You are a code optimization expert. Improve performance and efficiency.', timestamp: new Date() },
            { role: 'user', content: `Optimize this ${language} code for better performance:\n\n\`\`\`${language}\n${code}\n\`\`\``, timestamp: new Date() }
        ]);
        return response.message;
    }

    async review(code: string, language: string): Promise<string> {
        const response = await this.chat([
            { role: 'system', content: 'You are a senior code reviewer. Provide detailed code review feedback.', timestamp: new Date() },
            { role: 'user', content: `Review this ${language} code and provide feedback on best practices, potential issues, and improvements:\n\n\`\`\`${language}\n${code}\n\`\`\``, timestamp: new Date() }
        ]);
        return response.message;
    }

    async generateCommitMessage(): Promise<string> {
        const response = await this.chat([
            { role: 'system', content: 'You are a Git commit message expert. Generate concise, conventional commit messages.', timestamp: new Date() },
            { role: 'user', content: 'Generate a commit message for the current staged changes. Use conventional commits format (feat/fix/docs/style/refactor/test/chore).', timestamp: new Date() }
        ]);
        return response.message;
    }

    async getAvailableModels(): Promise<string[]> {
        try {
            const response = await this.httpClient.get('/api/models');
            return response.data.models.map((m: Model) => m.id);
        } catch {
            // Return default models if API unavailable
            return [
                'gpt-4-turbo',
                'gpt-4o',
                'gpt-3.5-turbo',
                'claude-3-opus',
                'claude-3-sonnet',
                'claude-3-haiku',
                'gemini-1.5-pro',
                'gemini-1.5-flash',
                'llama-3.1-70b',
                'llama-3.1-405b',
                'mixtral-8x7b',
                'qwen-2.5-72b',
                'gemma-4-27b'
            ];
        }
    }

    async getSkills(): Promise<Skill[]> {
        try {
            const response = await this.httpClient.get('/api/skills');
            return response.data.skills;
        } catch {
            return [];
        }
    }

    async setModel(model: string): Promise<void> {
        this.model = model;
    }

    getModel(): string {
        return this.model;
    }

    isConnected(): boolean {
        return this.connected;
    }

    disconnect(): void {
        if (this.ws) {
            this.ws.close();
            this.ws = null;
        }
    }
}
