import { create } from 'zustand';

interface ChatState {
  messages: Array<{
    id: string;
    role: 'user' | 'assistant' | 'system';
    content: string;
  }>;
  loading: boolean;
  addMessage: (role: 'user' | 'assistant' | 'system', content: string) => void;
  setLoading: (loading: boolean) => void;
  clearMessages: () => void;
}

export const useChatStore = create<ChatState>((set) => ({
  messages: [],
  loading: false,
  addMessage: (role, content) =>
    set((state) => ({
      messages: [
        ...state.messages,
        { id: Date.now().toString(), role, content },
      ],
    })),
  setLoading: (loading) => set({ loading }),
  clearMessages: () => set({ messages: [] }),
}));
