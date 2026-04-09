import { useState, useEffect, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { useChatStore } from './store';

interface Message {
  id: string;
  role: 'user' | 'assistant' | 'system';
  content: string;
  timestamp: Date;
}

interface ChannelInfo {
  name: string;
  connected: boolean;
  unread: number;
}

export default function App() {
  const [messages, setMessages] = useState<Message[]>([]);
  const [input, setInput] = useState('');
  const [loading, setLoading] = useState(false);
  const [voiceActive, setVoiceActive] = useState(false);
  const [channels, setChannels] = useState<ChannelInfo[]>([]);
  const [activeTab, setActiveTab] = useState<'chat' | 'channels' | 'skills' | 'settings'>('chat');
  const messagesEndRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // Load channels
    loadChannels();
    
    // Listen for voice events
    const unlisten = listen('voice:activated', () => {
      setVoiceActive(true);
    });
    
    return () => { unlisten.then(f => f()); };
  }, []);

  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  async function loadChannels() {
    try {
      const ch = await invoke<ChannelInfo[]>('get_channels');
      setChannels(ch);
    } catch (e) {
      console.error('Failed to load channels:', e);
    }
  }

  async function sendMessage() {
    if (!input.trim() || loading) return;
    
    const userMessage: Message = {
      id: Date.now().toString(),
      role: 'user',
      content: input,
      timestamp: new Date(),
    };
    
    setMessages(prev => [...prev, userMessage]);
    setInput('');
    setLoading(true);

    try {
      const response = await invoke<{ message: { content: string } }>('chat', {
        request: {
          messages: [...messages, userMessage].map(m => ({
            role: m.role,
            content: m.content,
          })),
        },
      });

      const assistantMessage: Message = {
        id: (Date.now() + 1).toString(),
        role: 'assistant',
        content: response.message.content,
        timestamp: new Date(),
      };
      
      setMessages(prev => [...prev, assistantMessage]);
    } catch (e) {
      console.error('Chat error:', e);
      setMessages(prev => [...prev, {
        id: (Date.now() + 1).toString(),
        role: 'system',
        content: `Hata: ${e}`,
        timestamp: new Date(),
      }]);
    } finally {
      setLoading(false);
    }
  }

  async function toggleVoice() {
    try {
      if (voiceActive) {
        await invoke('stop_voice');
      } else {
        await invoke('start_voice');
      }
      setVoiceActive(!voiceActive);
    } catch (e) {
      console.error('Voice toggle error:', e);
    }
  }

  return (
    <div className="app">
      {/* Sidebar */}
      <aside className="sidebar">
        <div className="logo">
          <span className="logo-icon">🧠</span>
          <span className="logo-text">SENTIENT</span>
        </div>
        
        <nav className="nav">
          <button 
            className={`nav-item ${activeTab === 'chat' ? 'active' : ''}`}
            onClick={() => setActiveTab('chat')}
          >
            💬 Sohbet
          </button>
          <button 
            className={`nav-item ${activeTab === 'channels' ? 'active' : ''}`}
            onClick={() => setActiveTab('channels')}
          >
            📡 Kanallar
            {channels.some(c => c.unread > 0) && (
              <span className="badge">{channels.reduce((a, c) => a + c.unread, 0)}</span>
            )}
          </button>
          <button 
            className={`nav-item ${activeTab === 'skills' ? 'active' : ''}`}
            onClick={() => setActiveTab('skills')}
          >
            🧩 Beceriler
          </button>
          <button 
            className={`nav-item ${activeTab === 'settings' ? 'active' : ''}`}
            onClick={() => setActiveTab('settings')}
          >
            ⚙️ Ayarlar
          </button>
        </nav>

        <div className="voice-section">
          <button 
            className={`voice-btn ${voiceActive ? 'active' : ''}`}
            onClick={toggleVoice}
          >
            {voiceActive ? '🎤 Dinliyorum...' : '🎤 Sesli Komut'}
          </button>
        </div>

        <div className="version">
          v0.1.0 • Rust + Tauri
        </div>
      </aside>

      {/* Main Content */}
      <main className="main">
        {activeTab === 'chat' && (
          <div className="chat-container">
            <div className="messages">
              {messages.length === 0 && (
                <div className="welcome">
                  <h1>Merhaba! Ben SENTIENT</h1>
                  <p>Çoklu-ajan AI işletim sistemi. Size nasıl yardımcı olabilirim?</p>
                </div>
              )}
              {messages.map(msg => (
                <div key={msg.id} className={`message ${msg.role}`}>
                  <div className="message-header">
                    <span className="role">
                      {msg.role === 'user' ? '👤 Siz' : '🧠 SENTIENT'}
                    </span>
                    <span className="time">
                      {msg.timestamp.toLocaleTimeString('tr-TR')}
                    </span>
                  </div>
                  <div className="message-content">{msg.content}</div>
                </div>
              ))}
              {loading && (
                <div className="message assistant">
                  <div className="message-content loading">
                    <span></span><span></span><span></span>
                  </div>
                </div>
              )}
              <div ref={messagesEndRef} />
            </div>
            
            <div className="input-area">
              <textarea
                value={input}
                onChange={e => setInput(e.target.value)}
                onKeyDown={e => {
                  if (e.key === 'Enter' && !e.shiftKey) {
                    e.preventDefault();
                    sendMessage();
                  }
                }}
                placeholder="Mesajınızı yazın... (Enter: gönder, Shift+Enter: yeni satır)"
                rows={3}
              />
              <button onClick={sendMessage} disabled={loading || !input.trim()}>
                {loading ? '⏳' : '➤'}
              </button>
            </div>
          </div>
        )}

        {activeTab === 'channels' && (
          <div className="channels-container">
            <h2>Kanallar</h2>
            <div className="channels-list">
              {channels.map(ch => (
                <div key={ch.name} className={`channel-card ${ch.connected ? 'connected' : ''}`}>
                  <span className="channel-icon">
                    {ch.name === 'telegram' ? '📱' : 
                     ch.name === 'discord' ? '🎮' : 
                     ch.name === 'whatsapp' ? '💬' : '📡'}
                  </span>
                  <span className="channel-name">{ch.name}</span>
                  <span className={`status ${ch.connected ? 'connected' : 'disconnected'}`}>
                    {ch.connected ? 'Bağlı' : 'Bağlı değil'}
                  </span>
                  {ch.unread > 0 && <span className="unread-badge">{ch.unread}</span>}
                </div>
              ))}
            </div>
          </div>
        )}

        {activeTab === 'skills' && (
          <div className="skills-container">
            <h2>Beceri Marketi</h2>
            <p>Yakında... (ClawHub uyumlu)</p>
          </div>
        )}

        {activeTab === 'settings' && (
          <div className="settings-container">
            <h2>Ayarlar</h2>
            <p>Yakında...</p>
          </div>
        )}
      </main>
    </div>
  );
}
