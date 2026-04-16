# ═══════════════════════════════════════════════════════════════════════════════
#  📱 SENTIENT OS — apps/ KULLANIM REHBERİ
# ═══════════════════════════════════════════════════════════════════════════════
#  Tarih: 2026-04-16 08:35 UTC
#  Hazırlayan: Pi (AI Agent)
#  Kaynak: Gerçek kaynak kodu incelenerek hazırlandı
# ═══════════════════════════════════════════════════════════════════════════════

# ═══════════════════════════════════════════════════════════════════════════════
#  GENEL MİMARİ
# ═══════════════════════════════════════════════════════════════════════════════

```
apps/
├── desktop/          ← Tauri v2 + React + Rust (masaüstü)
│   ├── src/          ← React/TypeScript frontend
│   └── src-tauri/    ← Rust backend (Tauri)
└── mobile/           ← Native mobil uygulama
    ├── android/      ← Kotlin + Jetpack Compose
    └── ios/          ← SwiftUI
```

```
┌─────────────────────────────────────────────────────────────┐
│                    SENTIENT OS Apps                          │
├──────────────────────┬──────────────────────────────────────┤
│   Desktop (Tauri)    │         Mobile (Native)              │
│                      │                                      │
│  ┌────────────────┐  │  ┌─────────────┐ ┌───────────────┐  │
│  │ React + TS     │  │  │ Kotlin      │ │ SwiftUI       │  │
│  │ (Frontend)     │  │  │ (Android)   │ │ (iOS)         │  │
│  └───────┬────────┘  │  └──────┬──────┘ └───────┬───────┘  │
│          │ Tauri IPC │         │ SDK       │ SDK           │
│  ┌───────▼────────┐  │  ┌──────▼──────────────────▼──────┐ │
│  │ Rust Backend   │  │  │    SENTIENT Core (Rust)        │ │
│  │ sentient_core  │  │  │    sentient_voice              │ │
│  │ sentient_voice │  │  │    sentient_channels           │ │
│  │ sentient_ch.   │  │  │    sentient_llm                │ │
│  └────────────────┘  │  └────────────────────────────────┘ │
└──────────────────────┴──────────────────────────────────────┘
```

---

# ═══════════════════════════════════════════════════════════════════════════════
#  1. DESKTOP — TAURI v2 + REACT
# ═══════════════════════════════════════════════════════════════════════════════

## Teknoloji Yığını

| Katman | Teknoloji | Versiyon |
|--------|-----------|----------|
| Framework | Tauri | v2.0 |
| Frontend | React + TypeScript | React 18, TS 5.5 |
| Build | Vite | v5.4 |
| State | Zustand | v4.5 |
| Backend | Rust (Tauri) | - |
| Notifications | tauri-plugin-notification | v2.0 |
| Shell | tauri-plugin-shell | v2.0 |
| FileSystem | tauri-plugin-fs | v2.0 |
| HTTP | tauri-plugin-http | v2.0 |
| WebSocket | tauri-plugin-websocket | v2.0 |

## Rust Crate Bağımlılıkları

```toml
# apps/desktop/src-tauri/Cargo.toml
sentient_core    = { path = "../../../crates/sentient_core" }
sentient_voice   = { path = "../../../crates/sentient_voice" }
sentient_channels = { path = "../../../crates/sentient_channels" }
```

## Dosya Yapısı

| Dosya | Satır | Açıklama |
|-------|-------|----------|
| `src-tauri/src/main.rs` | ~80 | Ana uygulama, 20 Tauri komutu kaydı |
| `src-tauri/src/commands.rs` | ~210 | Tüm komut implementasyonları |
| `src-tauri/src/tray.rs` | ~45 | Sistem tepsisi (Göster/Gizle/Ses/Çıkış) |
| `src-tauri/src/voice.rs` | ~35 | Ses dinleyici + TTS entegrasyonu |
| `src/App.tsx` | ~230 | React UI — 4 tab |
| `src/store.ts` | ~25 | Zustand state management |
| `src/main.tsx` | - | React entry point |
| `src/styles.css` | - | Dark tema CSS |
| `index.html` | - | HTML shell |
| `tauri.conf.json` | - | Tauri yapılandırma |

## 20 Tauri Komutu (commands.rs'den)

### Yapılandırma Komutları

| Komut | İmza | Ne Yapar? | Durum |
|-------|------|-----------|-------|
| `get_config` | `() → AppConfig` | Mevcut ayarları oku | ✅ Çalışıyor |
| `set_config` | `(AppConfig) → ()` | Ayarları güncelle | ⚠️ Diske kaydetmiyor |

### Chat Komutları

| Komut | İmza | Ne Yapar? | Durum |
|-------|------|-----------|-------|
| `chat` | `(ChatRequest) → ChatResponse` | OpenAI API'ye istek at | ✅ Çalışıyor |
| `chat_stream` | `(ChatRequest) → ()` | SSE streaming | ❌ TODO |
| `stop_generation` | `() → ()` | Devam eden isteği iptal et | ❌ TODO |

### Ses Komutları

| Komut | İmza | Ne Yapar? | Durum |
|-------|------|-----------|-------|
| `start_voice` | `() → ()` | Ses tanımayı başlat | ✅ State değiştirir |
| `stop_voice` | `() → ()` | Ses tanımayı durdur | ✅ State değiştirir |
| `get_voice_status` | `() → bool` | Ses aktif mi? | ✅ Çalışıyor |

### Kanal Komutları

| Komut | İmza | Ne Yapar? | Durum |
|-------|------|-----------|-------|
| `send_message` | `(channel, recipient, msg) → ()` | Kanala mesaj gönder | ❌ TODO |
| `get_channels` | `() → Vec<ChannelInfo>` | Kanalları listele | ⚠️ Mock data |
| `connect_channel` | `(channel) → ()` | Kanala bağlan | ❌ TODO |
| `disconnect_channel` | `(channel) → ()` | Kanaldan çık | ❌ TODO |

### Beceri Komutları

| Komut | İmza | Ne Yapar? | Durum |
|-------|------|-----------|-------|
| `search_skills` | `(query) → Vec<SkillInfo>` | Beceri ara | ⚠️ Mock data |
| `install_skill` | `(skill_id) → ()` | Beceri yükle | ❌ TODO |
| `list_installed_skills` | `() → Vec<SkillInfo>` | Yüklü beceriler | ⚠️ Boş döner |

### Sistem Komutları

| Komut | İmza | Ne Yapar? | Durum |
|-------|------|-----------|-------|
| `get_system_info` | `() → SystemInfo` | Versiyon, platform, arch | ✅ Çalışıyor |
| `open_logs` | `() → ()` | Log klasörünü aç | ❌ TODO |
| `check_updates` | `() → Option<String>` | Güncelleme kontrol | ❌ TODO |

## AppConfig Yapısı

```rust
pub struct AppConfig {
    pub api_key: Option<String>,     // LLM API anahtarı
    pub model: String,                // "gpt-4o" (varsayılan)
    pub voice_enabled: bool,          // true
    pub wake_word: String,            // "sentient"
    pub channels: Vec<String>,        // ["telegram", "discord"]
    pub theme: String,                // "dark"
    pub language: String,             // "tr"
}
```

## React Frontend — 4 Tab

### Tab 1: 💬 Sohbet
- Mesaj listesi (LazyVStack benzeri)
- Kullanıcı mesajı sağda (indigo), asistan solda (surface)
- Enter: gönder, Shift+Enter: yeni satır
- Loading animasyonu (3 bouncing dot)
- Auto-scroll

### Tab 2: 📡 Kanallar
- Telegram 📱, Discord 🎮, WhatsApp 💬
- Bağlı/Bağlı değil durumu
- Okunmamış mesaj badge'i
- Bağlan/Kopar butonları

### Tab 3: 🧩 Beceriler
- "Yakında... (ClawHub uyumlu)" yazısı
- Henüz implement edilmedi

### Tab 4: ⚙️ Ayarlar
- "Yakında..." yazısı
- Henüz implement edilmedi

## Sistem Tepsisi (tray.rs)

| Menü Öğesi | Aksiyon |
|-----------|--------|
| Göster | Ana pencereyi aç + odakla |
| Gizle | Ana pencereyi gizle |
| 🎤 Ses | Ses dinleyici toggle |
| Çıkış | Uygulamayı kapat (`app.exit(0)`) |

## Ses Entegrasyonu (voice.rs)

```rust
// start_voice_listener() → sonsuz döngü, wake word bekler
// process_voice_input(audio_data) → STT → metin
// speak(app, text) → TTS → frontend'e "voice:speak" event emit
```

## Build Çıktıları

| Platform | Format | Yol |
|----------|--------|-----|
| Linux | `.deb` + `.AppImage` | `target/release/bundle/` |
| macOS | `.dmg` | `target/release/bundle/` |
| Windows | `.msi` | `target/release/bundle/` |

## Linux Bağımlılıkları

```
libwebkit2gtk-4.1-0
libssl3
libgtk-3-0
```

---

# ═══════════════════════════════════════════════════════════════════════════════
#  2. MOBILE — ANDROID (Kotlin + Jetpack Compose)
# ═══════════════════════════════════════════════════════════════════════════════

## Dosya Yapısı

| Dosya | Satır | Açıklama |
|-------|-------|----------|
| `MainActivity.kt` | ~330 | Ana ekran + tüm Composable'lar |
| `SentientViewModel.kt` | ~55 | StateFlow ile mesaj/voice yönetimi |

## Teknoloji

| Katman | Teknoloji |
|--------|-----------|
| Dil | Kotlin |
| UI | Jetpack Compose + Material 3 |
| State | StateFlow + ViewModel |
| Mimari | MVVM |

## 4 Tab

### Tab 1: 💬 Sohbet (`ChatScreen`)
- `LazyColumn` + mesaj baloncukları
- Kullanıcı sağda (indigo %20), asistan solda (surfaceVariant)
- `OutlinedTextField` + `FilledIconButton` (gönder)
- Auto-scroll son mesaja
- Loading: `CircularProgressIndicator`

### Tab 2: 📡 Kanallar (`ChannelsScreen`)
- Kanal kartları: Telegram, Discord, WhatsApp, Signal
- Yeşil bağlantı durumu, kırmızı badge
- Her kart: ikon + isim + durum + okunmamış sayı

### Tab 3: 🎤 Ses (`VoiceScreen`)
- 200dp mikrofon butonu
- Radial gradient animasyon:
  - Dinlemede: Indigo → Mor
  - Dinlerken: Kırmızı → Turuncu
- `.scaleEffect(isListening ? 1.1 : 1.0)` spring animasyon
- 5 saniye otomatik durdurma (demo)
- "Uyandırma kelimesi: Hey SENTIENT"

### Tab 4: ⚙️ Ayarlar (`SettingsScreen`)
- API Key (SecureField yok — düz TextField)
- Server URL
- Voice toggle (Switch)
- Versiyon: 0.1.0

## SentientViewModel

```kotlin
class SentientViewModel : ViewModel() {
    val messages: StateFlow<List<ChatMessage>>   // mesaj listesi
    val loading: StateFlow<Boolean>              // loading durumu
    val isListening: StateFlow<Boolean>           // ses dinleme durumu

    fun sendMessage(text: String)                 // mesaj gönder (simüle)
    fun toggleVoice()                             // ses toggle (5sn auto-stop)
}
```

## UI Tema

```kotlin
// Dark renk paleti
primary = Color(0xFF6366F1)         // Indigo
primaryContainer = Color(0xFF4F46E5)
secondary = Color(0xFFA855F7)       // Mor
background = Color(0xFF0F0F1A)      // Çok koyu lacivert
surface = Color(0xFF1A1A2E)        // Koyu lacivert
```

---

# ═══════════════════════════════════════════════════════════════════════════════
#  3. MOBILE — iOS (SwiftUI)
# ═══════════════════════════════════════════════════════════════════════════════

## Dosya Yapısı

| Dosya | Satır | Açıklama |
|-------|-------|----------|
| `SentientApp.swift` | ~340 | Tüm iOS uygulaması |
| `Info.plist` | - | Uygulama meta verileri |

## Teknoloji

| Katman | Teknoloji |
|--------|-----------|
| Dil | Swift |
| UI | SwiftUI |
| State | @StateObject + @Published |
| Ağ | URLSessionWebSocketTask |
| Kalıcı | @AppStorage (UserDefaults) |

## 4 Tab

### Tab 1: 💬 Sohbet (`ChatView`)
- `ScrollView` + `LazyVStack` mesaj listesi
- Mesaj baloncukları: Kullanıcı sağda (indigo %20), asistan solda (secondary %10)
- `TextField` + gönder butonu (paperplane.fill)
- Auto-scroll son mesaja (withAnimation)
- Navigation title: "SENTIENT"

### Tab 2: 📡 Kanallar (`ChannelsView`)
- `List` ile kanal kartları
- SF Symbols: paperplane, gamecontroller, message, bubble.left.and.bubble.right
- Yeşil/kırmızı bağlantı durumu
- Kırmızı Capsule badge (okunmamış sayı)

### Tab 3: 🎤 Ses (`VoiceView`)
- 200pt Circle butonu
- Dinlemede: Indigo circle + mic.fill icon
- Dinlerken: Kırmızı circle + waveform icon
- `.scaleEffect(isListening ? 1.1 : 1.0)` spring animasyon
- 5 saniye otomatik durdurma
- "Uyandırma kelimesi: Hey SENTIENT"

### Tab 4: ⚙️ Ayarlar (`SettingsView`)
- `Form` yapısı
- API Key: `SecureField` (gerçek şifre gizleme!)
- Server URL: `TextField` (URL keyboard)
- Voice toggle: `Toggle`
- Dil seçimi: `Picker` (Türkçe / English / Deutsch)
- Hakkında: Versiyon, GitHub link, Dokümantasyon link

## AppState (Swift)

```swift
class AppState: ObservableObject {
    @Published var isConnected: Bool         // Gateway bağlantı durumu
    @Published var voiceActive: Bool         // Ses aktif mi?
    @Published var messages: [ChatMessage]   // Mesaj listesi
    @Published var apiKey: String            // API anahtarı
    @Published var serverURL: String         // "https://api.sentient.ai"

    // WebSocket bağlantı
    private var webSocketTask: URLSessionWebSocketTask?

    func connect()      // WebSocket başlat
    func disconnect()   // WebSocket kapat
    func sendMessage()  // Mesaj gönder (JSON format)
}
```

## Kalıcı Ayarlar (@AppStorage)

```swift
@AppStorage("apiKey") private var apiKey = ""
@AppStorage("serverURL") private var serverURL = "https://api.sentient.ai"
@AppStorage("voiceEnabled") private var voiceEnabled = true
@AppStorage("language") private var language = "tr"
```

---

# ═══════════════════════════════════════════════════════════════════════════════
#  4. ÖZELLİK KARŞILAŞTIRMA
# ═══════════════════════════════════════════════════════════════════════════════

| Özellik | Desktop (Tauri) | Android (Kotlin) | iOS (SwiftUI) |
|---------|-----------------|-------------------|---------------|
| UI/UX | ✅ Tam | ✅ Tam | ✅ Tam |
| Chat (API) | ✅ OpenAI endpoint | ⚠️ Simüle | ⚠️ WebSocket |
| Streaming chat | ❌ TODO | ❌ Yok | ❌ Yok |
| Voice STT | ⚠️ Scaffold | ⚠️ 5sn demo | ⚠️ Scaffold |
| Voice TTS | ⚠️ Scaffold | ❌ Yok | ❌ Yok |
| Telegram | ❌ TODO | ❌ Mock data | ❌ Mock data |
| Discord | ❌ TODO | ❌ Mock data | ❌ Mock data |
| Skills market | ❌ Mock | ❌ Yok | ❌ Yok |
| Config kaydetme | ❌ TODO | ❌ Yok | ✅ @AppStorage |
| Sistem tepsisi | ✅ Çalışıyor | ❌ N/A | ❌ N/A |
| Bildirimler | ✅ Plugin | ❌ Yok | ❌ Yok |
| Dark tema | ✅ CSS | ✅ Material3 | ✅ SwiftUI |
| Türkçe UI | ✅ | ✅ | ✅ |
| Build çıktısı | .msi/.dmg/.deb | .apk | .ipa |
| Gerçek SENTIENT core | ✅ Bağlı | ❌ Yok | ❌ Yok |

---

# ═══════════════════════════════════════════════════════════════════════════════
#  5. EVE GELİNCE KURULUM REHBERİ
# ═══════════════════════════════════════════════════════════════════════════════

## Desktop (Tauri) Kurulumu

### Linux Ön Koşulları

```bash
sudo apt install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev \
  librsvg2-dev libssl-dev libsoup-3.0-dev libjavascriptcoregtk-4.1-dev
```

### macOS Ön Koşulları

```bash
# Xcode Command Line Tools
xcode-select --install

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Windows Ön Koşulları

```powershell
# Microsoft C++ Build Tools
# https://visualstudio.microsoft.com/visual-cpp-build-tools/

# WebView2 (Windows 10/11'de zaten var)
```

### Build Adımları

```bash
cd apps/desktop

# 1. Node.js bağımlılıkları
npm install

# 2. Geliştirme modu (hot reload + Rust derleme)
npm run tauri:dev
# BEKLENEN: Pencere açılır, chat ekranı görünür, sohbet çalışır

# 3. Release build (installable)
npm run tauri:build
# ÇIKTI:
#   Linux:   target/release/bundle/deb/sentient-desktop_0.1.0_amd64.deb
#            target/release/bundle/appimage/sentient-desktop_0.1.0_amd64.AppImage
#   macOS:   target/release/bundle/dmg/SENTIENT_0.1.0_aarch64.dmg
#   Windows: target/release/bundle/msi/SENTIENT_0.1.0_x64_en-US.msi
```

### Doğrulama

```bash
# Pencere açıldı mı?
# Sohbet tab'ında mesaj yaz → yanıt geldi mi?
# Kanallar tab'ında Telegram/Discord görünüyor mu?
# Ses butonuna bas → "Dinliyorum..." yazısı çıktı mı?
# Sistem tepsisinde SENTIENT ikonu görünüyor mu?
# Gizle/Göster çalışıyor mu?
```

---

## Android Kurulumu

### Ön Koşulları

```bash
# Android Studio kur
# https://developer.android.com/studio

# Android SDK 34+
# Kotlin 2.0+

# Emulator veya gerçek cihaz
```

### Build Adımları

```bash
# 1. Android Studio'da aç:
#    File → Open → apps/mobile/android

# 2. Gradle sync bekleyin

# 3. Emulator oluştur:
#    Tools → Device Manager → Create Device
#    Pixel 7, API 34, 2048MB RAM

# 4. Run ▶️
#    BEKLENEN: Emulator'da SENTIENT uygulaması açılır
```

### Doğrulama

```
☐ Uygulama açılıyor mu?
☐ 4 tab görünüyor mu? (Sohbet, Kanallar, Ses, Ayarlar)
☐ Sohbet: Mesaj yaz → simüle yanıt geldi mi?
☐ Kanallar: Telegram/Discord kartları görünüyor mu?
☐ Ses: Mikrofon butonuna bas → kırmızı oldu mu? 5sn sonra geri döndü mü?
☐ Ayarlar: API Key alanı görünüyor mu?
☐ Dark tema aktif mi?
☐ Türkçe UI doğru mu?
```

---

## iOS Kurulumu

### Ön Koşulları

```bash
# Mac gerekli (Xcode sadece macOS'te çalışır)
# Xcode 16+
# iOS 17+ Simulator veya gerçek iPhone

# Xcode Command Line Tools
xcode-select --install
```

### Build Adımları

```bash
# 1. Xcode'da aç:
#    File → Open → apps/mobile/ios

# 2. Signing ayarla:
#    TARGETS → Signing & Capabilities → Team seç

# 3. Simulator oluştur:
#    Window → Devices and Simulators
#    iPhone 15 Pro, iOS 17

# 4. Run ▶️
#    BEKLENEN: Simulator'da SENTIENT uygulaması açılır
```

### Doğrulama

```
☐ Uygulama açılıyor mu?
☐ 4 tab görünüyor mu? (Sohbet, Kanallar, Ses, Ayarlar)
☐ Sohbet: Mesaj yaz → simüle yanıt geldi mi?
☐ Kanallar: Telegram/Discord kartları görünüyor mu?
☐ Ses: Mikrofon butonuna bas → kırmızı animasyon oldu mu?
☐ Ayarlar: API Key (SecureField), Server URL, Voice toggle
☐ Dil seçimi: Türkçe / English / Deutsch
☐ GitHub link tıklanabiliyor mu?
☐ Dark mode aktif mi?
☐ SwiftUI animasyonlar çalışıyor mu?
```

---

# ═══════════════════════════════════════════════════════════════════════════════
#  6. TODO — YAPILACAKLAR (EVE GELİNCE)
# ═══════════════════════════════════════════════════════════════════════════════

## 🔴 Yüksek Öncelik

| # | Görev | Platform | Zorluk | Açıklama |
|---|-------|----------|--------|----------|
| 1 | Chat → V-GATE proxy | Desktop | Orta | OpenAI yerine `localhost:1071/v1/chat/completions` |
| 2 | Chat → Gerçek API | Android | Orta | Retrofit/Ktor ile sentient gateway'e bağla |
| 3 | Chat → WebSocket | iOS | Orta | Gerçek gateway WebSocket'ine bağla |
| 4 | Streaming chat | Desktop | Orta | SSE EventSource ile streaming implement et |

## 🟡 Orta Öncelik

| # | Görev | Platform | Zorluk | Açıklama |
|---|-------|----------|--------|----------|
| 5 | Voice → sentient_voice | Desktop | Zor | Gerçek STT/TTS crate'ini çağır |
| 6 | Telegram SDK | Tümü | Zor | sentient_channels crate'inden bağla |
| 7 | Discord SDK | Tümü | Zor | sentient_channels crate'inden bağla |
| 8 | Skills → sentient_marketplace | Desktop | Orta | Gerçek beceri marketine bağla |
| 9 | Config → dosyaya kaydet | Desktop | Kolay | `$APPDATA/sentient/config.json` |
| 10 | Push bildirim | Mobil | Zor | FCM (Android) + APNs (iOS) |

## 🟢 Düşük Öncelik

| # | Görev | Platform | Zorluk | Açıklama |
|---|-------|----------|--------|----------|
| 11 | Biometrik giriş | Mobil | Orta | FaceID / parmak izi |
| 12 | Widget | Mobil | Zor | Ana ekranda mesaj widget'ı |
| 13 | Watch desteği | iOS | Zor | Apple Watch komplikasyonu |
| 14 | Accessibility | Tümü | Orta | VoiceOver / TalkBack desteği |
| 15 | i18n | Tümü | Orta | Dil değiştirme UI'ya bağla |

---

# ═══════════════════════════════════════════════════════════════════════════════
#  7. HIZLI BAŞLANGIÇ KOMUTLARI
# ═══════════════════════════════════════════════════════════════════════════════

## Desktop (tek komut)

```bash
cd apps/desktop && npm install && npm run tauri:dev
```

## Android (Android Studio)

```
File → Open → apps/mobile/android → Run ▶️
```

## iOS (Xcode)

```
File → Open → apps/mobile/ios → Run ▶️
```

---

*Son Güncelleme: 2026-04-16 08:35 UTC*
*Hazırlayan: Pi (AI Agent)*
*Kaynak: Gerçek kaynak kodu incelenerek — MainActivity.kt, SentientApp.swift, commands.rs, main.rs*
