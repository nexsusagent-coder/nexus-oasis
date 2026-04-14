# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS - EKSİKLİKLER VE ÖNCELİK LİSTESİ (TAM SÜRÜM)
# ═══════════════════════════════════════════════════════════════════════════════
#  Tarih: 2026-04-14
#  Kaynak: Oturum 9 Entegrasyon Testleri ve Kod Analizi
#  Amaç: OpenClaw ve benzerlerine gerçek rakip olmak için gerekenler
#  Güncelleme: Gerçek test ortamı, ücretsiz alternatifler, somut çözümler eklendi
# ═══════════════════════════════════════════════════════════════════════════════

---

# 📊 MEVCUT DURUM ÖZETİ

## İstatistikler
- **Toplam Crate:** 87 adet
- **Toplam Satır:** ~288,000
- **TODO/FIXME:** 86 adet
- **Simülasyon/Stub:** 262 yer
- **Gerçek Çalışan:** ~5 crate
- **Test Edilmemiş:** ~70+ crate

## ÖNEMLİ NOT: Gerçek Test Ortamı

**Kullanıcı Sorusu:** *"Bu sistemi gerçek bir bilgisayarda test etsek bazı eksiklikler giderilebilir mi?"*

**Cevap:** EVET, birçok "simülasyon" aslında gerçek ortamda çalışabilir çünkü:

1. **Kod yazıldı ama test edilmedi** - Çoğu crate için gerçek implementasyon kodu var, sadece biz sunucuda test edemedik
2. **Bağımlılıklar eksik** - Chromium, Tesseract gibi bağımlılıklar kurulu olsa çalışabilir
3. **API key gerekiyor** - Bazı özellikler sadece API key ile çalışıyor

### Gerçek Bilgisayarda Test İçin Gerekenler

```bash
# ═══════════════════════════════════════════════════════════════
#  ADIM ADIM GERÇEK TEST ORTAMI KURULUMU
# ═══════════════════════════════════════════════════════════════

# 1. Sistem bağımlılıkları (Ubuntu/Debian)
sudo apt update
sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    chromium-browser \      # Browser otomasyonu için
    tesseract-ocr \         # OCR için
    tesseract-ocr-eng \     # İngilizce dil
    tesseract-ocr-tur \     # Türkçe dil
    portaudio19-dev \       # Ses kayıt için
    libasound2-dev \        # ALSA (ses)
    ffmpeg \                # Ses/video işleme
    libgtk-3-dev \          # GUI için
    libwebkit2gtk-4.0-dev   # Web view için

# 2. Ollama (Lokal LLM - Ücretsiz)
curl -fsSL https://ollama.com/install.sh | sh
ollama pull llama3.2        # 3B model (~2GB)
ollama pull gemma3:27b      # 27B model (~16GB) - önerilen
ollama pull deepseek-r1:67b # 67B model (~40GB) - en iyi

# 3. Whisper.cpp (Lokal STT - Ücretsiz)
git clone https://github.com/ggerganov/whisper.cpp
cd whisper.cpp
make
./models/download-ggml-model.sh base.en  # ~75MB
# Türkçe için:
./models/download-ggml-model.sh medium   # ~1.5GB

# 4. Piper TTS (Lokal TTS - Ücretsiz)
wget https://github.com/rhasspy/piper/releases/download/v1.2.0/piper_1.2.0_amd64.tar.gz
tar -xvf piper_1.2.0_amd64.tar.gz
./piper/piper --model tr_TR-fettah-medium.onnx --output_file test.wav

# 5. Rust derleme
cd SENTIENT_CORE
cargo build --release

# 6. Çalıştır
./target/release/sentient gateway
```

---

# 🔴 KRİTİK ÖNCELİK (P0) - Temel İşlevler

## P0-1: Voice (STT/TTS) - ÜCRETSİZ AÇIK KAYNAK ÇÖZÜMLER

**Kullanıcı Sorusu:** *"Voice kısmında ücretli değil ücretsiz açık kaynak projeleri sisteme eklesek nasıl olur?"*

**Cevap:** MÜKEMMEL FİKİR! Aşağıda tüm ücretsiz alternatifler var:

### ════════════════════════════════════════════════════════════════════════════
###  SEÇENEK 1: Whisper.cpp (ÖNERİLEN - Tamamen Lokal, Ücretsiz)
### ════════════════════════════════════════════════════════════════════════════

**Avantajları:**
- ✅ %100 ücretsiz
- ✅ Tamamen lokal (internet gerektirmez)
- ✅ GPU desteği (CUDA, Metal, ROCm)
- ✅ 100+ dil desteği (Türkçe dahil)
- ✅ OpenAI Whisper ile aynı kalite
- ✅ Real-time streaming desteği

**Kurulum:**
```bash
# Ubuntu/Debian
git clone https://github.com/ggerganov/whisper.cpp
cd whisper.cpp

# CPU için
make

# GPU (CUDA) için
make WHISPER_CUBLAS=1

# GPU (Apple Silicon) için
make WHISPER_METAL=1

# Modelleri indir
bash ./models/download-ggml-model.sh base.en      # ~75MB  - Hızlı
bash ./models/download-ggml-model.sh small.en     # ~466MB - Dengeli
bash ./models/download-ggml-model.sh medium       # ~1.5GB - Kaliteli
bash ./models/download-ggml-model.sh large-v3     # ~2.9GB - En iyi

# Türkçe için medium veya large önerilir
```

**Rust Entegrasyonu:**
```rust
// sentient_voice/src/stt/whisper_cpp.rs

use std::process::Command;

pub struct WhisperCpp {
    model_path: String,
    whisper_binary: String,
}

impl WhisperCpp {
    pub fn new(model_path: &str) -> Self {
        Self {
            model_path: model_path.to_string(),
            whisper_binary: "/usr/local/bin/whisper-cpp".to_string(),
        }
    }

    /// Ses dosyasını metne çevir
    pub async fn transcribe(&self, audio_path: &str) -> Result<String, VoiceError> {
        let output = Command::new(&self.whisper_binary)
            .args([
                "-m", &self.model_path,
                "-f", audio_path,
                "-l", "tr",           // Türkçe
                "-ot",                 // Sadece metin çıktısı
                "--output-csv",        // CSV formatı
            ])
            .output()?;

        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            Ok(self.parse_whisper_output(&text))
        } else {
            Err(VoiceError::TranscriptionFailed)
        }
    }

    /// Real-time streaming (microphone)
    pub async fn transcribe_stream(&self) -> Result<Receiver<String>, VoiceError> {
        // Whisper.cpp streaming modu
        let mut child = Command::new(&self.whisper_binary)
            .args([
                "-m", &self.model_path,
                "-l", "tr",
                "--stream",           // Streaming mod
                "-t", "8",            // 8 thread
            ])
            .stdout(Stdio::piped())
            .spawn()?;

        let (tx, rx) = channel();
        
        // stdout'u oku ve parse et
        tokio::spawn(async move {
            let reader = BufReader::new(child.stdout.take().unwrap());
            for line in reader.lines() {
                if let Ok(text) = line {
                    let _ = tx.send(text);
                }
            }
        });

        Ok(rx)
    }

    fn parse_whisper_output(&self, output: &str) -> String {
        // Whisper çıktısından metin kısmını ayıkla
        output
            .lines()
            .filter(|l| !l.starts_with('['))  // Timestamp'leri atla
            .collect::<Vec<_>>()
            .join(" ")
    }
}

impl Default for WhisperCpp {
    fn default() -> Self {
        Self::new("/usr/share/whisper-models/ggml-medium.bin")
    }
}
```

**Kullanım:**
```rust
// sentient_voice/src/lib.rs güncelleme

pub enum SttProvider {
    WhisperCpp(WhisperCpp),
    Vosk(VoskEngine),
    OpenAI(OpenAIWhisper),  // Ücretli alternatif
}

impl VoiceEngine {
    pub fn new_local() -> Self {
        // Önce lokal Whisper.cpp dene
        let whisper = WhisperCpp::default();
        Self {
            stt: SttProvider::WhisperCpp(whisper),
            tts: TtsProvider::Piper(PiperTts::default()),
        }
    }
}
```

---

### ════════════════════════════════════════════════════════════════════════════
###  SEÇENEK 2: Vosk (Offline, Hızlı, Türkçe Destekli)
### ════════════════════════════════════════════════════════════════════════════

**Avantajları:**
- ✅ %100 ücretsiz
- ✅ Tamamen offline
- ✅ Düşük kaynak tüketimi (Raspberry Pi'de bile çalışır)
- ✅ Türkçe model var (~50MB)
- ✅ Real-time streaming için optimize

**Dezavantajları:**
- ⚠️ Whisper'dan daha düşük doğruluk
- ⚠️ Kelime hazinesi sınırlı

**Kurulum:**
```bash
# Rust crate
cargo add vosk

# Türkçe model indir
wget https://alphacephei.com/vosk/models/vosk-model-tr-0.1.zip
unzip vosk-model-tr-0.1.zip
mv vosk-model-tr-0.1 ~/.local/share/vosk-models/tr
```

**Rust Entegrasyonu:**
```rust
// sentient_voice/src/stt/vosk.rs

use vosk::{Model, Decoder};

pub struct VoskEngine {
    model: Model,
}

impl VoskEngine {
    pub fn new(model_path: &str) -> Result<Self, VoiceError> {
        let model = Model::new(model_path)
            .ok_or(VoiceError::ModelNotFound)?;
        Ok(Self { model })
    }

    pub fn transcribe(&self, audio: &[i16], sample_rate: f32) -> String {
        let mut decoder = Decoder::new(&self.model).unwrap();
        decoder.accept_waveform(audio, sample_rate as i32);
        
        let mut result = String::new();
        while let Some(text) = decoder.partial_result() {
            result.push_str(text);
        }
        result
    }
}

impl Default for VoskEngine {
    fn default() -> Self {
        Self::new(&std::env::var("VOSK_MODEL_PATH")
            .unwrap_or_else(|_| "~/.local/share/vosk-models/tr".into()))
            .expect("Vosk model yüklenemedi")
    }
}
```

---

### ════════════════════════════════════════════════════════════════════════════
###  SEÇENEK 3: Piper TTS (Lokal, Hızlı, Ücretsiz TTS)
### ════════════════════════════════════════════════════════════════════════════

**Avantajları:**
- ✅ %100 ücretsiz
- ✅ Tamamen lokal
- ✅ Çok hızlı (real-time'dan hızlı)
- ✅ Türkçe ses modeli var
- ✅ Düşük kaynak tüketimi

**Kurulum:**
```bash
# Binary indir
wget https://github.com/rhasspy/piper/releases/download/v1.2.0/piper_1.2.0_amd64.tar.gz
tar -xvf piper_1.2.0_amd64.tar.gz
sudo mv piper/piper /usr/local/bin/

# Türkçe model indir
mkdir -p ~/.local/share/piper/models
cd ~/.local/share/piper/models
wget https://huggingface.co/rhasspy/piper-voices/resolve/main/tr/tr_TR/medium/tr_TR-medium.onnx
wget https://huggingface.co/rhasspy/piper-voices/resolve/main/tr/tr_TR/medium/tr_TR-medium.onnx.json
```

**Rust Entegrasyonu:**
```rust
// sentient_voice/src/tts/piper.rs

use std::process::Command;
use tokio::process::Command as AsyncCommand;

pub struct PiperTts {
    model_path: String,
    piper_binary: String,
}

impl PiperTts {
    pub fn new() -> Self {
        Self {
            model_path: dirs::data_dir()
                .unwrap()
                .join("piper/models/tr_TR-medium.onnx")
                .to_str()
                .unwrap()
                .to_string(),
            piper_binary: "/usr/local/bin/piper".to_string(),
        }
    }

    /// Metni sese çevir
    pub async fn synthesize(&self, text: &str) -> Result<Vec<u8>, VoiceError> {
        let output = AsyncCommand::new(&self.piper_binary)
            .args([
                "--model", &self.model_path,
                "--output-raw",  // Raw PCM çıktısı
                "--quiet",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        // Metni stdin'e yaz
        if let Some(mut stdin) = output.stdin.take() {
            use tokio::io::AsyncWriteExt;
            stdin.write_all(text.as_bytes()).await?;
        }

        // PCM çıktısını al
        let result = output.wait_with_output().await?;
        Ok(result.stdout)
    }

    /// Dosyaya kaydet
    pub async fn synthesize_to_file(&self, text: &str, output_path: &str) -> Result<(), VoiceError> {
        let status = AsyncCommand::new(&self.piper_binary)
            .args([
                "--model", &self.model_path,
                "--output_file", output_path,
            ])
            .stdin(Stdio::piped())
            .spawn()?
            .stdin
            .take()
            .unwrap()
            .write_all(text.as_bytes())
            .await?;

        Ok(())
    }
}

impl Default for PiperTts {
    fn default() -> Self {
        Self::new()
    }
}
```

---

### ════════════════════════════════════════════════════════════════════════════
###  SEÇENEK 4: Coqui TTS (Python bazlı, yüksek kalite)
### ════════════════════════════════════════════════════════════════════════════

**Avantajları:**
- ✅ Ücretsiz ve açık kaynak
- ✅ Yüksek kalite ses sentezi
- ✅ Voice cloning desteği
- ✅ Çok dilli modeller

**Kurulum:**
```bash
pip install TTS

# Türkçe model ile test
tts --text "Merhaba dünya" --out_path output.wav \
    --model_name tts_models/tr/common-voice/glow-tts
```

**Rust Entegrasyonu (Python bridge):**
```rust
// sentient_voice/src/tts/coqui.rs

use std::process::Command;

pub struct CoquiTts;

impl CoquiTts {
    pub async fn synthesize(&self, text: &str, output_path: &str) -> Result<(), VoiceError> {
        let status = Command::new("tts")
            .args([
                "--text", text,
                "--out_path", output_path,
                "--model_name", "tts_models/tr/common-voice/glow-tts",
            ])
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err(VoiceError::SynthesisFailed)
        }
    }
}
```

---

### ════════════════════════════════════════════════════════════════════════════
###  SES SİSTEMİ KARŞILAŞTIRMA TABLOSU
### ════════════════════════════════════════════════════════════════════════════

| Özellik | Whisper.cpp | Vosk | OpenAI API |
|---------|-------------|------|------------|
| **Fiyat** | ✅ Ücretsiz | ✅ Ücretsiz | ❌ $0.006/dk |
| **İnternet** | ✅ Gerekmez | ✅ Gerekmez | ❌ Gerekir |
| **Türkçe Kalite** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **Hız** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **Kaynak** | 2-4GB RAM | 500MB RAM | API |
| **Real-time** | ✅ Evet | ✅ Evet | ❌ Hayır |

| Özellik | Piper TTS | Coqui TTS | ElevenLabs |
|---------|-----------|-----------|------------|
| **Fiyat** | ✅ Ücretsiz | ✅ Ücretsiz | ❌ $5+/ay |
| **İnternet** | ✅ Gerekmez | ✅ Gerekmez | ❌ Gerekir |
| **Türkçe** | ✅ Var | ✅ Var | ⚠️ Sınırlı |
| **Hız** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ |
| **Kalite** | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |

---

### ════════════════════════════════════════════════════════════════════════════
###  ÖNERİLEN YAPILANDIRMA (Tamamen Ücretsiz)
### ════════════════════════════════════════════════════════════════════════════

```rust
// sentient_voice/src/lib.rs

pub struct VoiceConfig {
    /// STT: Whisper.cpp (en iyi kalite)
    pub stt_provider: SttProvider,
    /// TTS: Piper (en hızlı) veya Coqui (en kaliteli)
    pub tts_provider: TtsProvider,
    /// Wake word detection (isteğe bağlı)
    pub wake_word: Option<WakeWordConfig>,
}

impl Default for VoiceConfig {
    fn default() -> Self {
        Self {
            stt_provider: SttProvider::WhisperCpp(WhisperCpp::default()),
            tts_provider: TtsProvider::Piper(PiperTts::default()),
            wake_word: None,
        }
    }
}

/// Tamamen lokal voice pipeline
pub struct LocalVoicePipeline {
    whisper: WhisperCpp,
    piper: PiperTts,
}

impl LocalVoicePipeline {
    pub async fn process_voice_input(&self, audio: &[u8]) -> Result<String, VoiceError> {
        // 1. Ses -> Metin (Whisper.cpp)
        let text = self.whisper.transcribe(audio).await?;
        log::info!("📝 Transcribed: {}", text);
        
        // 2. Metin -> LLM -> Cevap
        let response = self.llm.chat(&text).await?;
        
        // 3. Cevap -> Ses (Piper)
        let audio_response = self.piper.synthesize(&response).await?;
        
        // 4. Ses çal
        self.play_audio(&audio_response)?;
        
        Ok(response)
    }
}
```

---

## P0-2: Browser - Gerçek Web Otomasyonu

**Mevcut Durum:** Simülasyon

**GERÇEK ÇÖZÜM İÇİN YAPILACAKLAR:**

### Adım 1: Chromium Kurulumu

```bash
# Ubuntu/Debian
sudo apt install chromium-browser

# veya Google Chrome
wget https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb
sudo dpkg -i google-chrome-stable_current_amd64.deb

# Chromedriver (Selenium için)
sudo apt install chromium-chromedriver
```

### Adım 2: Rust Bağımlılıkları

```toml
# sentient_browser/Cargo.toml
[dependencies]
headless_chrome = "1.0"
thirtyfour = "0.31"  # Selenium WebDriver
fantoccini = "0.19" # Async WebDriver
```

### Adım 3: Gerçek Implementasyon

```rust
// sentient_browser/src/engine/chrome.rs

use headless_chrome::{Browser, protocol::page::ScreenshotFormat};
use std::sync::Arc;

pub struct ChromeEngine {
    browser: Arc<Browser>,
}

impl ChromeEngine {
    pub fn new() -> Result<Self, BrowserError> {
        let browser = Browser::new(
            headless_chrome::LaunchOptions {
                headless: true,
                executable_path: Some("/usr/bin/chromium-browser".into()),
                ..Default::default()
            }
        )?;
        
        Ok(Self {
            browser: Arc::new(browser),
        })
    }

    /// Sayfayı aç
    pub async fn navigate(&self, url: &str) -> Result<(), BrowserError> {
        let tab = self.browser.new_tab()?;
        tab.navigate_to(url)?;
        tab.wait_until_navigated()?;
        Ok(())
    }

    /// Element bul ve tıkla
    pub async fn click(&self, selector: &str) -> Result<(), BrowserError> {
        let tab = self.browser.active_tab()?;
        tab.wait_for_element(selector)?.click()?;
        Ok(())
    }

    /// Metin yaz
    pub async fn type_text(&self, selector: &str, text: &str) -> Result<(), BrowserError> {
        let tab = self.browser.active_tab()?;
        let element = tab.wait_for_element(selector)?;
        element.type_into(text)?;
        Ok(())
    }

    /// Sayfa içeriğini al
    pub async fn get_content(&self) -> Result<String, BrowserError> {
        let tab = self.browser.active_tab()?;
        let html = tab.get_content()?;
        Ok(html)
    }

    /// Screenshot al
    pub async fn screenshot(&self) -> Result<Vec<u8>, BrowserError> {
        let tab = self.browser.active_tab()?;
        let png = tab.capture_screenshot(ScreenshotFormat::PNG, None, true)?;
        Ok(png)
    }
}
```

### Adım 4: Test Komutları

```bash
# Browser test
cd SENTIENT_CORE
cargo test -p sentient_browser --test browser_integration

# Manuel test
./target/release/sentient browser navigate "https://example.com"
./target/release/sentient browser click "#button-id"
./target/release/sentient browser screenshot
```

---

## P0-3: Vision - Gerçek OCR

**Mevcut Durum:** Simülasyon

**GERÇEK ÇÖZÜM İÇİN YAPILACAKLAR:**

### Adım 1: Tesseract Kurulumu

```bash
# Ubuntu/Debian
sudo apt install tesseract-ocr tesseract-ocr-eng tesseract-ocr-tur

# Ek diller
sudo apt install tesseract-ocr-deu  # Almanca
sudo apt install tesseract-ocr-fra  # Fransızca
```

### Adım 2: Rust Bağımlılıkları

```toml
# sentient_vision/Cargo.toml
[dependencies]
tesseract = "0.12"
image = "0.24"
leptess = "0.1"  # Tesseract wrapper
opencv = "0.86"  # Görüntü işleme (isteğe bağlı)
```

### Adım 3: Gerçek Implementasyon

```rust
// sentient_vision/src/ocr/tesseract.rs

use leptess::LepTess;
use image::DynamicImage;

pub struct TesseractEngine {
    api: LepTess,
}

impl TesseractEngine {
    pub fn new() -> Result<Self, VisionError> {
        let api = LepTess::new(None, "tur+eng")?;  // Türkçe + İngilizce
        Ok(Self { api })
    }

    /// Görselden metin çıkar
    pub fn extract_text(&mut self, image: &DynamicImage) -> Result<String, VisionError> {
        // Görseli belleğe al
        let img_bytes = vec![];
        image.write_to(&mut Cursor::new(&mut img_bytes), image::ImageFormat::Png)?;
        
        // OCR yap
        self.api.set_image_from_mem(&img_bytes)?;
        let text = self.api.get_utf8_text()?;
        
        Ok(text)
    }

    /// Dosyadan metin çıkar
    pub fn extract_text_from_file(&mut self, path: &str) -> Result<String, VisionError> {
        self.api.set_image(path)?;
        let text = self.api.get_utf8_text()?;
        Ok(text)
    }

    /// Bounding box'lar ile metin çıkar
    pub fn extract_text_with_boxes(&mut self, path: &str) -> Result<Vec<TextRegion>, VisionError> {
        self.api.set_image(path)?;
        
        let mut regions = vec![];
        let boxes = self.api.get_component_boxes(leptess::PageIteratorLevel::Word, true)?;
        
        for bbox in boxes {
            regions.push(TextRegion {
                text: bbox.clone(),
                x: bbox.x,
                y: bbox.y,
                width: bbox.w,
                height: bbox.h,
                confidence: self.api.mean_text_conf()? as f32 / 100.0,
            });
        }
        
        Ok(regions)
    }
}

#[derive(Debug, Clone)]
pub struct TextRegion {
    pub text: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub confidence: f32,
}
```

---

## P0-4: Desktop Kontrol - Gerçek Klavye/Fare

**Mevcut Durum:** Simülasyon

**GERÇEK ÇÖZÜM İÇİN YAPILACAKLAR:**

### Adım 1: Rust Bağımlılıkları

```toml
# sentient_desktop/Cargo.toml
[dependencies]
enigo = "0.2"        # Klavye/fare kontrolü
screenshots = "0.5"  # Ekran görüntüsü
x11 = { version = "2.21", optional = true }  # Linux X11
rdev = "0.5"         # Global input hooking
```

### Adım 2: Gerçek Implementasyon

```rust
// sentient_desktop/src/mouse.rs

use enigo::{Enigo, MouseControllable, MouseButton};

pub struct MouseController {
    enigo: Enigo,
}

impl MouseController {
    pub fn new() -> Result<Self, DesktopError> {
        Ok(Self {
            enigo: Enigo::new(),
        })
    }

    /// Fareyi hareket ettir
    pub fn move_to(&mut self, x: i32, y: i32) -> Result<(), DesktopError> {
        self.enigo.mouse_move_to(x, y);
        log::info!("🖱️  Mouse moved to ({}, {})", x, y);
        Ok(())
    }

    /// Tıkla
    pub fn click(&mut self, button: MouseButton) -> Result<(), DesktopError> {
        self.enigo.mouse_click(button);
        log::info!("🖱️  Mouse clicked: {:?}", button);
        Ok(())
    }

    /// Çift tıkla
    pub fn double_click(&mut self) -> Result<(), DesktopError> {
        self.enigo.mouse_click(MouseButton::Left);
        std::thread::sleep(std::time::Duration::from_millis(100));
        self.enigo.mouse_click(MouseButton::Left);
        Ok(())
    }

    /// Scroll
    pub fn scroll(&mut self, amount: i32) -> Result<(), DesktopError> {
        self.enigo.mouse_scroll_x(amount);
        Ok(())
    }
}

// sentient_desktop/src/keyboard.rs

use enigo::{Enigo, KeyboardControllable, Key};

pub struct KeyboardController {
    enigo: Enigo,
}

impl KeyboardController {
    pub fn new() -> Result<Self, DesktopError> {
        Ok(Self {
            enigo: Enigo::new(),
        })
    }

    /// Metin yaz
    pub fn type_text(&mut self, text: &str) -> Result<(), DesktopError> {
        self.enigo.key_sequence(text);
        log::info!("⌨️  Typed: {} chars", text.len());
        Ok(())
    }

    /// Tuş bas
    pub fn key_press(&mut self, key: Key) -> Result<(), DesktopError> {
        self.enigo.key_click(key);
        Ok(())
    }

    /// Kısayol (Ctrl+C, Ctrl+V vs.)
    pub fn shortcut(&mut self, keys: &[Key]) -> Result<(), DesktopError> {
        for key in keys {
            self.enigo.key_down(*key);
        }
        std::thread::sleep(std::time::Duration::from_millis(50));
        for key in keys.iter().rev() {
            self.enigo.key_up(*key);
        }
        Ok(())
    }
}

// sentient_desktop/src/screen.rs

use screenshots::Screenshots;

pub struct ScreenCapture;

impl ScreenCapture {
    /// Ekran görüntüsü al
    pub fn capture_all() -> Result<Vec<u8>, DesktopError> {
        let screens = Screenshots::new().ok_or(DesktopError::NoScreen)?;
        let screen = screens.iter().next().ok_or(DesktopError::NoScreen)?;
        
        let image = screen.capture()?;
        let png = image.to_png()?;
        
        Ok(png)
    }

    /// Belirli bölgeyi yakala
    pub fn capture_region(x: i32, y: i32, width: u32, height: u32) -> Result<Vec<u8>, DesktopError> {
        let screens = Screenshots::new().ok_or(DesktopError::NoScreen)?;
        let screen = screens.iter().next().ok_or(DesktopError::NoScreen)?;
        
        let image = screen.capture_area(x, y, width, height)?;
        let png = image.to_png()?;
        
        Ok(png)
    }
}
```

---

# 🟠 YÜKSEK ÖNCELİK (P1) - Profesyonel Özellikler

## P1-1: Channels - Gerçek Bot Entegrasyonları

**Her platform için yapılacaklar:**

### Telegram Bot

```bash
# 1. Bot oluştur
# @BotFather'a /newcommand gönder

# 2. Token al
export TELEGRAM_BOT_TOKEN="123456789:ABCdefGHIjklMNOpqrsTUVwxyz"

# 3. Test
curl "https://api.telegram.org/bot$TELEGRAM_BOT_TOKEN/getMe"
```

```rust
// sentient_channels/src/telegram.rs

use teloxide::{prelude::*, utils::command::BotCommands};

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    #[command(description = "Yardım")]
    Help,
    #[command(description = "Sohbet")]
    Chat(String),
}

pub async fn run_telegram_bot(token: String) {
    let bot = Bot::new(token);
    
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        // LLM'e gönder
        let response = send_to_llm(&msg.text().unwrap_or("")).await;
        bot.send_message(msg.chat.id, response).await?;
        Ok(())
    }).await;
}
```

### Discord Bot

```bash
# 1. Discord Developer Portal'da uygulama oluştur
# 2. Bot oluştur ve token al
export DISCORD_BOT_TOKEN="your-token"

# 3. Sunucuya ekle (OAuth2 URL)
```

```rust
// sentient_channels/src/discord.rs

use serenity::{prelude::*, model::prelude::*, framework::standard::*};

pub async fn run_discord_bot(token: String) {
    let mut client = Client::builder(token)
        .framework(StandardFramework::new())
        .await
        .expect("Discord client oluşturulamadı");
    
    client.start().await.expect("Discord bot başlatılamadı");
}
```

---

# 📋 SOMUT YAPILACAKLAR LİSTESİ

**Kullanıcı Sorusu:** *"Eksiklik dediklerini kaydettin ama gidermek için ne yapılacak, ne lazım onları da ekledin mi?"*

**Cevap:** Aşağıda her eksiklik için SOMUT ADIMLAR var:

## ✅ Hemen Yapılabilir (0-2 saat)

| # | Görev | Komut/Dosya | Durum |
|---|-------|-------------|-------|
| 1 | Tesseract kur | `sudo apt install tesseract-ocr` | Hazır |
| 2 | Chromium kur | `sudo apt install chromium-browser` | Hazır |
| 3 | Whisper.cpp derle | `git clone + make` | Hazır |
| 4 | Piper TTS kur | Binary indir | Hazır |
| 5 | Ollama kur | `curl -fsSL https://ollama.com/install.sh \| sh` | Hazır |

## 🔧 Kod Değişikliği Gerektiren (2-8 saat)

| # | Görev | Dosya | Satır Sayısı |
|---|-------|-------|--------------|
| 1 | Whisper.cpp entegrasyonu | `sentient_voice/src/stt/whisper_cpp.rs` | ~150 |
| 2 | Piper TTS entegrasyonu | `sentient_voice/src/tts/piper.rs` | ~100 |
| 3 | Chrome browser engine | `sentient_browser/src/engine/chrome.rs` | ~200 |
| 4 | Tesseract OCR | `sentient_vision/src/ocr/tesseract.rs` | ~100 |
| 5 | Enigo mouse/keyboard | `sentient_desktop/src/mouse.rs` | ~100 |

## 📦 Yeni Bağımlılıklar

```toml
# Cargo.toml'a eklenecekler

# Voice
vosk = "0.2"          # STT (alternatif)
whisper-rs = "0.8"    # Whisper bind

# Browser
headless_chrome = "1.0"
thirtyfour = "0.31"

# Vision
tesseract = "0.12"
leptess = "0.1"

# Desktop
enigo = "0.2"
rdev = "0.5"
screenshots = "0.5"

# Channels
teloxide = "0.12"     # Telegram
serenity = "0.11"     # Discord
slack-morphism = "1.0" # Slack
```

---

# 📅 ÖNERİLEN ROADMAP (GÜNCELLENMİŞ)

## Hafta 1: Lokal Voice (Tamamen Ücretsiz)
- [ ] Whisper.cpp entegrasyonu (4 saat)
- [ ] Piper TTS entegrasyonu (3 saat)
- [ ] Voice pipeline testi (2 saat)

## Hafta 2: Browser + Vision
- [ ] Chrome headless entegrasyonu (6 saat)
- [ ] Tesseract OCR (4 saat)
- [ ] Görüntü ön işleme (3 saat)

## Hafta 3: Desktop + Channels
- [ ] Klavye/fare kontrolü (4 saat)
- [ ] Telegram bot (3 saat)
- [ ] Discord bot (3 saat)

## Hafta 4: Entegrasyon + Test
- [ ] Tüm modülleri birleştir (8 saat)
- [ ] Integration testleri (8 saat)
- [ ] Dokümantasyon (4 saat)

---

# 💰 MALİYET (GÜNCELLENMİŞ - ÜCRETSİZ SEÇENEKLER)

| Özellik | Ücretli | Ücretsiz Alternatif |
|---------|---------|---------------------|
| STT | OpenAI Whisper ($0.006/dk) | Whisper.cpp ($0) |
| TTS | ElevenLabs ($5+/ay) | Piper TTS ($0) |
| LLM | OpenAI API ($) | Ollama + Llama3 ($0) |
| OCR | GPT-4 Vision ($) | Tesseract ($0) |
| Browser | - | Chromium ($0) |
| Desktop | - | Enigo ($0) |

**TOPLAM: $0/ay** (tamamen lokal, açık kaynak)

---

# 🔧 HIZLI BAŞLANGIÇ SCRİPTİ

```bash
#!/bin/bash
# setup-local-voice.sh - Tüm ücretsiz voice araçlarını kur

set -e

echo "🎙️  SENTIENT Lokal Voice Kurulumu"

# 1. Sistem bağımlılıkları
echo "📦 Sistem bağımlılıkları kurulumu..."
sudo apt update
sudo apt install -y build-essential portaudio19-dev libasound2-dev ffmpeg

# 2. Whisper.cpp
echo "🔊 Whisper.cpp kurulumu..."
if [ ! -d "whisper.cpp" ]; then
    git clone https://github.com/ggerganov/whisper.cpp
fi
cd whisper.cpp
make
bash ./models/download-ggml-model.sh medium
cd ..

# 3. Piper TTS
echo "🗣️  Piper TTS kurulumu..."
wget -q https://github.com/rhasspy/piper/releases/download/v1.2.0/piper_1.2.0_amd64.tar.gz
tar -xzf piper_1.2.0_amd64.tar.gz
sudo mv piper/piper /usr/local/bin/
mkdir -p ~/.local/share/piper/models
cd ~/.local/share/piper/models
wget -q https://huggingface.co/rhasspy/piper-voices/resolve/main/tr/tr_TR/medium/tr_TR-medium.onnx
wget -q https://huggingface.co/rhasspy/piper-voices/resolve/main/tr/tr_TR/medium/tr_TR-medium.onnx.json
cd -

# 4. Ollama
echo "🦙 Ollama kurulumu..."
curl -fsSL https://ollama.com/install.sh | sh
ollama pull llama3.2

# 5. Rust bağımlılıkları
echo "🦀 Rust crate'leri..."
cd SENTIENT_CORE
cargo add vosk enigo tesseract leptess headless_chrome

echo "✅ Kurulum tamamlandı!"
echo ""
echo "Test için:"
echo "  ./whisper.cpp/main -m whisper.cpp/models/ggml-medium.bin -f test.wav"
echo "  echo 'Merhaba' | piper --model ~/.local/share/piper/models/tr_TR-medium.onnx -f test.wav"
```

---

*Dosya sürümü: 2.0*
*Son güncelleme: 2026-04-14*
*Güncellemeler: Gerçek test ortamı, ücretsiz voice alternatifleri, somut çözüm adımları eklendi*
