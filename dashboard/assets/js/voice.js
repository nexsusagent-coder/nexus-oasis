/**
 * ═══════════════════════════════════════════════════════════════════════════════
 *  SENTIENT Voice Client - JARVIS-level Voice Interface
 * ═══════════════════════════════════════════════════════════════════════════════
 * 
 *  Web tarayıcısından sesli iletişim sağlar:
 *  - Mikrofon kaydı ve streaming
 *  - WebSocket ile gerçek zamanlı iletişim
 *  - Wake word detection
 *  - Visual feedback (waveform)
 *  - Auto-reconnect
 */

class SentientVoiceClient {
    constructor(options = {}) {
        // Configuration
        this.wsUrl = options.wsUrl || `ws://${window.location.host}/ws/voice`;
        this.sampleRate = options.sampleRate || 16000;
        this.language = options.language || 'tr';
        this.wakeWordEnabled = options.wakeWordEnabled !== false;
        
        // State
        this.isConnected = false;
        this.isListening = false;
        this.isProcessing = false;
        this.isSpeaking = false;
        this.sessionId = null;
        this.chunkSeq = 0;
        
        // Audio
        this.audioContext = null;
        this.mediaStream = null;
        this.audioProcessor = null;
        this.analyser = null;
        this.recorder = null;
        
        // WebSocket
        this.ws = null;
        this.reconnectAttempts = 0;
        this.maxReconnectAttempts = 5;
        this.reconnectDelay = 1000;
        
        // Callbacks
        this.onTranscript = options.onTranscript || (() => {});
        this.onPartialTranscript = options.onPartialTranscript || (() => {});
        this.onLlmResponse = options.onLlmResponse || (() => {});
        this.onAudioResponse = options.onAudioResponse || (() => {});
        this.onStateChange = options.onStateChange || (() => {});
        this.onError = options.onError || (() => {});
        this.onConnected = options.onConnected || (() => {});
        this.onDisconnected = options.onDisconnected || (() => {});
        
        // Visual elements
        this.waveformCanvas = options.waveformCanvas || null;
        this.waveformCtx = null;
        this.animationFrame = null;
        
        // Audio playback queue
        this.audioQueue = [];
        this.isPlaying = false;
        
        // Auto-initialize if requested
        if (options.autoInit !== false) {
            this.init();
        }
    }
    
    /**
     * Initialize audio context and connect
     */
    async init() {
        try {
            // Request microphone permission
            await navigator.mediaDevices.getUserMedia({ audio: true });
            
            // Create audio context
            this.audioContext = new (window.AudioContext || window.webkitAudioContext)({
                sampleRate: this.sampleRate
            });
            
            log.info('🎙️ Sentient Voice Client initialized');
            
            // Connect WebSocket
            this.connect();
            
            return true;
        } catch (error) {
            log.error('Failed to initialize voice client:', error);
            this.onError({ code: 'INIT_ERROR', message: error.message });
            return false;
        }
    }
    
    /**
     * Connect to WebSocket
     */
    connect() {
        if (this.ws && this.ws.readyState === WebSocket.OPEN) {
            return;
        }
        
        log.info('🎙️ Connecting to voice WebSocket:', this.wsUrl);
        
        this.ws = new WebSocket(this.wsUrl);
        
        this.ws.onopen = () => {
            this.isConnected = true;
            this.reconnectAttempts = 0;
            log.info('🎙️ Voice WebSocket connected');
            this.onConnected();
            this.updateState('idle');
        };
        
        this.ws.onmessage = (event) => {
            this.handleMessage(event.data);
        };
        
        this.ws.onclose = (event) => {
            this.isConnected = false;
            this.isListening = false;
            log.warn('🎙️ Voice WebSocket closed:', event.code, event.reason);
            this.onDisconnected();
            this.updateState('disconnected');
            
            // Auto-reconnect
            if (this.reconnectAttempts < this.maxReconnectAttempts) {
                this.reconnectAttempts++;
                setTimeout(() => this.connect(), this.reconnectDelay * this.reconnectAttempts);
            }
        };
        
        this.ws.onerror = (error) => {
            log.error('🎙️ Voice WebSocket error:', error);
            this.onError({ code: 'WS_ERROR', message: 'WebSocket connection error' });
        };
    }
    
    /**
     * Disconnect WebSocket
     */
    disconnect() {
        if (this.ws) {
            this.ws.close(1000, 'User disconnect');
            this.ws = null;
        }
        this.isConnected = false;
    }
    
    /**
     * Handle incoming WebSocket message
     */
    handleMessage(data) {
        try {
            const msg = JSON.parse(data);
            
            switch (msg.type) {
                case 'stream_started':
                    this.sessionId = msg.session_id;
                    log.info('🎙️ Voice session started:', this.sessionId);
                    break;
                    
                case 'voice_activity_start':
                    this.updateState('listening');
                    break;
                    
                case 'voice_activity_end':
                    this.updateState('processing');
                    break;
                    
                case 'partial_transcript':
                    this.onPartialTranscript(msg);
                    break;
                    
                case 'transcript':
                    log.info('🎙️ Transcript:', msg.text);
                    this.onTranscript(msg);
                    break;
                    
                case 'llm_response':
                    this.onLlmResponse(msg);
                    break;
                    
                case 'audio_response':
                    this.playAudioResponse(msg.data, msg.format);
                    this.onAudioResponse(msg);
                    break;
                    
                case 'status':
                    this.updateState(msg.state);
                    break;
                    
                case 'error':
                    log.error('🎙️ Server error:', msg.code, msg.message);
                    this.onError(msg);
                    break;
                    
                case 'pong':
                    // Heartbeat response
                    break;
                    
                default:
                    log.debug('🎙️ Unknown message type:', msg.type);
            }
        } catch (error) {
            log.error('Failed to parse message:', error);
        }
    }
    
    /**
     * Send message to server
     */
    send(msg) {
        if (this.ws && this.ws.readyState === WebSocket.OPEN) {
            this.ws.send(JSON.stringify(msg));
        } else {
            log.warn('Cannot send: WebSocket not connected');
        }
    }
    
    /**
     * Start listening (microphone on)
     */
    async startListening() {
        if (this.isListening) return;
        
        try {
            // Resume audio context if suspended
            if (this.audioContext.state === 'suspended') {
                await this.audioContext.resume();
            }
            
            // Get media stream
            this.mediaStream = await navigator.mediaDevices.getUserMedia({
                audio: {
                    echoCancellation: true,
                    noiseSuppression: true,
                    autoGainControl: true,
                    sampleRate: this.sampleRate
                }
            });
            
            // Create audio source
            const source = this.audioContext.createMediaStreamSource(this.mediaStream);
            
            // Create analyser for visualization
            this.analyser = this.audioContext.createAnalyser();
            this.analyser.fftSize = 2048;
            source.connect(this.analyser);
            
            // Create script processor for audio processing
            // Note: ScriptProcessorNode is deprecated but still widely supported
            const bufferSize = 4096;
            this.audioProcessor = this.audioContext.createScriptProcessor(bufferSize, 1, 1);
            
            this.audioProcessor.onaudioprocess = (event) => {
                if (!this.isListening) return;
                
                const inputData = event.inputBuffer.getChannelData(0);
                const audioData = this.resampleIfNeeded(inputData);
                
                // Send to server
                this.sendAudioChunk(audioData);
                
                // Update visualization
                if (this.waveformCanvas) {
                    this.drawWaveform(inputData);
                }
            };
            
            source.connect(this.audioProcessor);
            this.audioProcessor.connect(this.audioContext.destination);
            
            this.isListening = true;
            this.chunkSeq = 0;
            
            // Send start message
            this.send({
                type: 'start_stream',
                sample_rate: this.sampleRate,
                language: this.language,
                wake_word: this.wakeWordEnabled
            });
            
            this.updateState('listening');
            log.info('🎙️ Started listening');
            
        } catch (error) {
            log.error('Failed to start listening:', error);
            this.onError({ code: 'MIC_ERROR', message: error.message });
        }
    }
    
    /**
     * Stop listening (microphone off)
     */
    stopListening() {
        if (!this.isListening) return;
        
        this.isListening = false;
        
        // Send stop message
        this.send({ type: 'stop_stream' });
        
        // Disconnect audio nodes
        if (this.audioProcessor) {
            this.audioProcessor.disconnect();
            this.audioProcessor = null;
        }
        
        if (this.mediaStream) {
            this.mediaStream.getTracks().forEach(track => track.stop());
            this.mediaStream = null;
        }
        
        // Stop visualization
        if (this.animationFrame) {
            cancelAnimationFrame(this.animationFrame);
            this.animationFrame = null;
        }
        
        this.updateState('idle');
        log.info('🎙️ Stopped listening');
    }
    
    /**
     * Toggle listening state
     */
    toggleListening() {
        if (this.isListening) {
            this.stopListening();
        } else {
            this.startListening();
        }
    }
    
    /**
     * Send audio chunk to server
     */
    sendAudioChunk(audioData) {
        // Convert to 16-bit PCM
        const pcmData = new Int16Array(audioData.length);
        for (let i = 0; i < audioData.length; i++) {
            pcmData[i] = Math.max(-32768, Math.min(32767, audioData[i] * 32767));
        }
        
        // Convert to base64
        const base64 = this.arrayBufferToBase64(pcmData.buffer);
        
        this.send({
            type: 'audio_chunk',
            data: base64,
            seq: this.chunkSeq++
        });
    }
    
    /**
     * Convert ArrayBuffer to Base64
     */
    arrayBufferToBase64(buffer) {
        const bytes = new Uint8Array(buffer);
        let binary = '';
        for (let i = 0; i < bytes.byteLength; i++) {
            binary += String.fromCharCode(bytes[i]);
        }
        return btoa(binary);
    }
    
    /**
     * Convert Base64 to ArrayBuffer
     */
    base64ToArrayBuffer(base64) {
        const binary = atob(base64);
        const bytes = new Uint8Array(binary.length);
        for (let i = 0; i < binary.length; i++) {
            bytes[i] = binary.charCodeAt(i);
        }
        return bytes.buffer;
    }
    
    /**
     * Resample audio if needed
     */
    resampleIfNeeded(audioData) {
        // If the audio context sample rate doesn't match our target, we'd resample here
        // For now, assume 16kHz
        return audioData;
    }
    
    /**
     * Play audio response from server
     */
    async playAudioResponse(base64Data, format) {
        try {
            const arrayBuffer = this.base64ToArrayBuffer(base64Data);
            
            // Decode audio data
            const audioBuffer = await this.audioContext.decodeAudioData(arrayBuffer);
            
            // Create and play source
            const source = this.audioContext.createBufferSource();
            source.buffer = audioBuffer;
            source.connect(this.audioContext.destination);
            
            this.isSpeaking = true;
            this.updateState('speaking');
            
            source.onended = () => {
                this.isSpeaking = false;
                this.updateState('idle');
            };
            
            source.start(0);
            
        } catch (error) {
            log.error('Failed to play audio:', error);
        }
    }
    
    /**
     * Send text for TTS
     */
    speakText(text, voiceId = null) {
        this.send({
            type: 'text_to_speech',
            text: text,
            voice_id: voiceId
        });
    }
    
    /**
     * Draw waveform visualization
     */
    drawWaveform(audioData) {
        if (!this.waveformCanvas) return;
        
        const ctx = this.waveformCtx || (this.waveformCtx = this.waveformCanvas.getContext('2d'));
        const canvas = this.waveformCanvas;
        
        const width = canvas.width;
        const height = canvas.height;
        
        ctx.fillStyle = '#1a1a2e';
        ctx.fillRect(0, 0, width, height);
        
        // Draw waveform
        ctx.lineWidth = 2;
        ctx.strokeStyle = this.isListening ? '#00ff88' : '#666';
        ctx.beginPath();
        
        const sliceWidth = width / audioData.length;
        let x = 0;
        
        for (let i = 0; i < audioData.length; i++) {
            const v = audioData[i];
            const y = (v + 1) * height / 2;
            
            if (i === 0) {
                ctx.moveTo(x, y);
            } else {
                ctx.lineTo(x, y);
            }
            
            x += sliceWidth;
        }
        
        ctx.stroke();
        
        // Draw center line
        ctx.strokeStyle = '#333';
        ctx.beginPath();
        ctx.moveTo(0, height / 2);
        ctx.lineTo(width, height / 2);
        ctx.stroke();
    }
    
    /**
     * Update state
     */
    updateState(newState) {
        this.state = newState;
        this.onStateChange(newState);
    }
    
    /**
     * Ping server to keep connection alive
     */
    ping() {
        this.send({ type: 'ping' });
    }
    
    /**
     * Destroy client
     */
    destroy() {
        this.stopListening();
        this.disconnect();
        this.audioContext = null;
    }
}

// Export for use
if (typeof module !== 'undefined' && module.exports) {
    module.exports = SentientVoiceClient;
}

// Global instance for inline usage
let sentientVoice = null;

/**
 * Initialize voice client with UI
 */
function initVoiceUI() {
    const canvas = document.getElementById('voiceWaveform');
    const micButton = document.getElementById('micButton');
    const transcriptEl = document.getElementById('transcript');
    const responseEl = document.getElementById('response');
    const statusEl = document.getElementById('voiceStatus');
    
    if (!canvas) {
        log.warn('Voice UI elements not found');
        return;
    }
    
    sentientVoice = new SentientVoiceClient({
        wsUrl: `ws://${window.location.host}/ws/voice`,
        waveformCanvas: canvas,
        
        onConnected: () => {
            if (statusEl) statusEl.textContent = 'Bağlı';
            if (statusEl) statusEl.className = 'status connected';
        },
        
        onDisconnected: () => {
            if (statusEl) statusEl.textContent = 'Bağlantı kesildi';
            if (statusEl) statusEl.className = 'status disconnected';
        },
        
        onStateChange: (state) => {
            const stateLabels = {
                'idle': 'Hazır',
                'listening': 'Dinliyor...',
                'processing': 'İşleniyor...',
                'speaking': 'Konuşuyor',
                'error': 'Hata'
            };
            
            if (statusEl) {
                statusEl.textContent = stateLabels[state] || state;
                statusEl.className = 'status ' + state;
            }
            
            if (micButton) {
                micButton.className = 'mic-button ' + state;
            }
        },
        
        onPartialTranscript: (msg) => {
            if (transcriptEl) {
                transcriptEl.textContent = msg.text + '...';
                transcriptEl.className = 'transcript partial';
            }
        },
        
        onTranscript: (msg) => {
            if (transcriptEl) {
                transcriptEl.textContent = msg.text;
                transcriptEl.className = 'transcript final';
            }
        },
        
        onLlmResponse: (msg) => {
            if (responseEl) {
                responseEl.textContent = msg.text;
                responseEl.className = 'response ' + (msg.is_streaming ? 'streaming' : 'final');
            }
        },
        
        onError: (error) => {
            log.error('Voice error:', error);
            if (statusEl) {
                statusEl.textContent = 'Hata: ' + error.message;
                statusEl.className = 'status error';
            }
        }
    });
    
    // Mic button click handler
    if (micButton) {
        micButton.addEventListener('click', () => {
            sentientVoice.toggleListening();
        });
    }
    
    // Keyboard shortcut (Space to toggle)
    document.addEventListener('keydown', (e) => {
        if (e.code === 'Space' && e.target === document.body) {
            e.preventDefault();
            sentientVoice.toggleListening();
        }
    });
    
    // Auto-ping every 30 seconds
    setInterval(() => {
        if (sentientVoice.isConnected) {
            sentientVoice.ping();
        }
    }, 30000);
}

// Initialize on DOM ready
document.addEventListener('DOMContentLoaded', () => {
    initVoiceUI();
});
