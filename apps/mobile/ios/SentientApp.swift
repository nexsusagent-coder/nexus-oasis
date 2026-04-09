import SwiftUI
import AVFoundation

/// SENTIENT iOS Application
@main
struct SentientApp: App {
    @StateObject private var appState = AppState()
    
    var body: some Scene {
        WindowGroup {
            MainView()
                .environmentObject(appState)
                .preferredColorScheme(.dark)
        }
    }
}

/// Application State
class AppState: ObservableObject {
    @Published var isConnected = false
    @Published var voiceActive = false
    @Published var messages: [ChatMessage] = []
    @Published var apiKey: String = ""
    @Published var serverURL: String = "https://api.sentient.ai"
    
    private var webSocketTask: URLSessionWebSocketTask?
    
    func connect() {
        guard let url = URL(string: serverURL) else { return }
        
        // WebSocket connection
        let request = URLRequest(url: url.appendingPathComponent("ws"))
        webSocketTask = URLSession.shared.webSocketTask(with: request)
        webSocketTask?.resume()
        
        isConnected = true
    }
    
    func disconnect() {
        webSocketTask?.cancel()
        isConnected = false
    }
    
    func sendMessage(_ text: String) {
        let message = ChatMessage(
            id: UUID().uuidString,
            role: "user",
            content: text,
            timestamp: Date()
        )
        messages.append(message)
        
        // Send via WebSocket
        let wsMessage = URLSessionWebSocketTask.Message.string(
            #"{"type":"chat","content":"\#(text)"}"#
        )
        webSocketTask?.send(wsMessage) { error in
            if let error = error {
                print("Send error: \(error)")
            }
        }
    }
}

/// Chat Message Model
struct ChatMessage: Identifiable, Codable {
    let id: String
    let role: String
    let content: String
    let timestamp: Date
    
    var isUser: Bool { role == "user" }
}

/// Main View
struct MainView: View {
    @EnvironmentObject var appState: AppState
    @State private var selectedTab = 0
    
    var body: some View {
        TabView(selection: $selectedTab) {
            ChatView()
                .tabItem {
                    Label("Sohbet", systemImage: "bubble.left.and.bubble.right")
                }
                .tag(0)
            
            ChannelsView()
                .tabItem {
                    Label("Kanallar", systemImage: "antenna.radiowaves.left.and.right")
                }
                .tag(1)
            
            VoiceView()
                .tabItem {
                    Label("Ses", systemImage: "mic.circle")
                }
                .tag(2)
            
            SettingsView()
                .tabItem {
                    Label("Ayarlar", systemImage: "gearshape")
                }
                .tag(3)
        }
        .tint(.indigo)
    }
}

/// Chat View
struct ChatView: View {
    @EnvironmentObject var appState: AppState
    @State private var inputText = ""
    @FocusState private var isInputFocused: Bool
    
    var body: some View {
        NavigationStack {
            VStack {
                // Messages
                ScrollViewReader { proxy in
                    ScrollView {
                        LazyVStack(spacing: 12) {
                            ForEach(appState.messages) { message in
                                MessageBubble(message: message)
                                    .id(message.id)
                            }
                        }
                        .padding()
                    }
                    .onChange(of: appState.messages.count) {
                        if let last = appState.messages.last {
                            withAnimation {
                                proxy.scrollTo(last.id, anchor: .bottom)
                            }
                        }
                    }
                }
                
                // Input
                HStack(spacing: 12) {
                    TextField("Mesajınızı yazın...", text: $inputText, axis: .vertical)
                        .textFieldStyle(.roundedBorder)
                        .focused($isInputFocused)
                        .onSubmit {
                            sendMessage()
                        }
                    
                    Button(action: sendMessage) {
                        Image(systemName: "paperplane.fill")
                            .font(.title3)
                    }
                    .buttonStyle(.borderedProminent)
                    .disabled(inputText.trimmingCharacters(in: .whitespaces).isEmpty)
                }
                .padding()
            }
            .navigationTitle("SENTIENT")
            .navigationBarTitleDisplayMode(.inline)
        }
    }
    
    func sendMessage() {
        let text = inputText.trimmingCharacters(in: .whitespaces)
        guard !text.isEmpty else { return }
        
        appState.sendMessage(text)
        inputText = ""
        isInputFocused = false
    }
}

/// Message Bubble
struct MessageBubble: View {
    let message: ChatMessage
    
    var body: some View {
        HStack {
            if message.isUser { Spacer() }
            
            VStack(alignment: message.isUser ? .trailing : .leading, spacing: 4) {
                Text(message.content)
                    .padding(.horizontal, 16)
                    .padding(.vertical, 10)
                    .background(
                        message.isUser
                            ? Color.indigo.opacity(0.2)
                            : Color.secondary.opacity(0.1)
                    )
                    .foregroundColor(.primary)
                    .clipShape(RoundedRectangle(cornerRadius: 16))
                
                Text(message.timestamp, style: .time)
                    .font(.caption2)
                    .foregroundStyle(.secondary)
            }
            
            if !message.isUser { Spacer() }
        }
    }
}

/// Channels View
struct ChannelsView: View {
    @EnvironmentObject var appState: AppState
    
    let channels = [
        ("Telegram", "paperplane", true, 5),
        ("Discord", "gamecontroller", true, 3),
        ("WhatsApp", "message", false, 0),
        ("Signal", "bubble.left.and.bubble.right", false, 0),
    ]
    
    var body: some View {
        NavigationStack {
            List {
                ForEach(channels, id: \.0) { channel in
                    HStack {
                        Image(systemName: channel.1)
                            .font(.title2)
                            .foregroundStyle(channel.2 ? .green : .secondary)
                            .frame(width: 40)
                        
                        Text(channel.0)
                            .font(.headline)
                        
                        Spacer()
                        
                        if channel.2 {
                            Text("Bağlı")
                                .font(.caption)
                                .foregroundStyle(.green)
                        } else {
                            Text("Bağlı değil")
                                .font(.caption)
                                .foregroundStyle(.secondary)
                        }
                        
                        if channel.3 > 0 {
                            Text("\(channel.3)")
                                .font(.caption)
                                .padding(.horizontal, 8)
                                .padding(.vertical, 4)
                                .background(.red)
                                .foregroundColor(.white)
                                .clipShape(Capsule())
                        }
                    }
                    .padding(.vertical, 8)
                }
            }
            .navigationTitle("Kanallar")
        }
    }
}

/// Voice View
struct VoiceView: View {
    @EnvironmentObject var appState: AppState
    @State private var isListening = false
    
    var body: some View {
        NavigationStack {
            VStack(spacing: 40) {
                // Waveform visualization
                Circle()
                    .fill(isListening ? Color.red : Color.indigo)
                    .frame(width: 200, height: 200)
                    .overlay(
                        Image(systemName: isListening ? "waveform" : "mic.fill")
                            .font(.system(size: 60))
                            .foregroundColor(.white)
                    )
                    .scaleEffect(isListening ? 1.1 : 1.0)
                    .animation(.spring(response: 0.5), value: isListening)
                    .onTapGesture {
                        isListening.toggle()
                        if isListening {
                            startListening()
                        }
                    }
                
                Text(isListening ? "Dinliyorum..." : "Sesli komut için dokunun")
                    .font(.title3)
                    .foregroundStyle(.secondary)
                
                VStack(spacing: 8) {
                    Text("Uyandırma kelimesi:")
                        .font(.headline)
                    Text("\"Hey SENTIENT\"")
                        .font(.title2)
                        .fontWeight(.bold)
                        .foregroundStyle(.indigo)
                }
                
                Spacer()
            }
            .padding()
            .navigationTitle("Ses")
        }
    }
    
    func startListening() {
        // Start speech recognition
        // After 5 seconds, stop for demo
        DispatchQueue.main.asyncAfter(deadline: .now() + 5) {
            isListening = false
        }
    }
}

/// Settings View
struct SettingsView: View {
    @EnvironmentObject var appState: AppState
    @AppStorage("apiKey") private var apiKey = ""
    @AppStorage("serverURL") private var serverURL = "https://api.sentient.ai"
    @AppStorage("voiceEnabled") private var voiceEnabled = true
    @AppStorage("language") private var language = "tr"
    
    var body: some View {
        NavigationStack {
            Form {
                Section("API") {
                    SecureField("API Key", text: $apiKey)
                        .textContentType(.password)
                    
                    TextField("Server URL", text: $serverURL)
                        .textContentType(.URL)
                        .autocapitalization(.none)
                        .keyboardType(.URL)
                }
                
                Section("Ses") {
                    Toggle("Ses Etkin", isOn: $voiceEnabled)
                }
                
                Section("Dil") {
                    Picker("Dil", selection: $language) {
                        Text("Türkçe").tag("tr")
                        Text("English").tag("en")
                        Text("Deutsch").tag("de")
                    }
                }
                
                Section("Hakkında") {
                    HStack {
                        Text("Versiyon")
                        Spacer()
                        Text("0.1.0")
                            .foregroundStyle(.secondary)
                    }
                    
                    Link("GitHub", destination: URL(string: "https://github.com/nexsusagent-coder/SENTIENT_CORE")!)
                    Link("Dokümantasyon", destination: URL(string: "https://docs.sentient.ai")!)
                }
            }
            .navigationTitle("Ayarlar")
        }
    }
}

#Preview {
    MainView()
        .environmentObject(AppState())
}
