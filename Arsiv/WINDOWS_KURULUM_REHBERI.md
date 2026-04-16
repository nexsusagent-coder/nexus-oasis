# ═══════════════════════════════════════════════════════════════════════════════
#  🪟 WINDOWS KURULUM REHBERİ — SENTIENT OS
# ═══════════════════════════════════════════════════════════════════════════════
#  Tarih: 2026-04-16
#  Hazırlayan: Pi (AI Agent)
#  Platform: Windows 10/11 x64
#  Güncelleme: PyO3 optional yapıldı, warning'ler düzeltildi
# ═══════════════════════════════════════════════════════════════════════════════

Bu rehber, Windows'ta SENTIENT OS kurulumunda karşılaşılan sorunları çözmek için
hazırlanmıştır.

---

# ═══════════════════════════════════════════════════════════════════════════════
#  BÖLÜM 1: HIZLI ÇÖZÜM — PyO3 Hatası
# ═══════════════════════════════════════════════════════════════════════════════

## Hata Mesajı

```
error: failed to run custom build command for `pyo3-build-config v0.25.1`
```

## Neden Olur?

PyO3, Rust ile Python arasında köprü kurar. Windows'ta bu köprü için:
1. Python kurulu olmalı
2. Python development headers olmalı
3. MSVC build tools olmalı
4. Environment variables doğru ayarlanmalı

## Çözüm 1: PyO3'suz Derle (ÖNERİLEN)

SENTIENT'in çoğu özelliği PyO3 gerektirmez. PyO3 sadece `sentient_python` crate'i
için gerekli, ve bu crate şu an devre dışı.

### Adım 1: Repoyu Güncelle

```powershell
cd C:\Users\AI_SYSTEM\Desktop\sentient-core
git pull
```

### Adım 2: Temiz Derleme

```powershell
# Önceki derlemeyi temizle
cargo clean

# Release derle
cargo build --release

# BEKLENEN: Hatasız derleme, ~5-10 dk
# Binary: target\release\sentient.exe
```

### Adım 3: Doğrulama

```powershell
.\target\release\sentient.exe --version
# BEKLENEN: sentient 4.0.0
```

---

## Çözüm 2: PyO3'ü Aktif Et (Python Entegrasyonu İstiyorsan)

Eğer Python entegrasyonu kesinlikle gerekirse:

### Adım 1: Python Kur

```powershell
# Yöntem 1: Winget ile (önerilen)
winget install Python.Python.3.12

# Yöntem 2: https://python.org/downloads/
# Python 3.12.x 64-bit indir
# ⚠️ "Add Python to PATH" işaretle!
```

### Adım 2: Python Doğrula

```powershell
python --version
# BEKLENEN: Python 3.12.x

python -c "import sys; print(sys.prefix)"
# BEKLENEN: C:\Users\AI_SYSTEM\AppData\Local\Programs\Python\Python312
```

### Adım 3: Visual Studio Build Tools Kur

```powershell
# Yöntem 1: Winget ile
winget install Microsoft.VisualStudio.2022.BuildTools --override "--add Microsoft.VisualStudio.Workload.VCTools --passive"

# Yöntem 2: Manuel
# https://visualstudio.microsoft.com/visual-cpp-build-tools/
# İndir → "Desktop development with C++" işaretle → Kur
```

### Adım 4: Environment Variables

```powershell
# PowerShell'de ayarla (geçici)
$env:PYTHON_SYS_EXECUTABLE = "C:\Users\AI_SYSTEM\AppData\Local\Programs\Python\Python312\python.exe"

# Kalıcı için:
[System.Environment]::SetEnvironmentVariable("PYTHON_SYS_EXECUTABLE", "C:\Users\AI_SYSTEM\AppData\Local\Programs\Python\Python312\python.exe", "User")
```

### Adım 5: Cargo.toml'u Düzenle

```powershell
# Cargo.toml'da pyo3 satırını uncomment et
# VEYA sentient_python crate'ini uncomment et
```

### Adım 6: Derle

```powershell
cargo clean
cargo build --release
```

---

# ═══════════════════════════════════════════════════════════════════════════════
#  BÖLÜM 2: YAYGIN WINDOWS HATALARI VE ÇÖZÜMLERİ
# ═══════════════════════════════════════════════════════════════════════════════

## Hata 1: "rustc not found"

```powershell
# Rust kur
winget install Rustlang.Rustup

# Yeni PowerShell aç
rustc --version
```

## Hata 2: "linker 'link.exe' not found"

```powershell
# Visual Studio Build Tools kur (yukarıda)
# VEYA Microsoft C++ Build Tools kur
```

## Hata 3: "cargo: not found"

```powershell
# Rustup'ı yeniden başlat
. $env:USERPROFILE\.cargo\env.ps1

# Doğrula
cargo --version
```

## Hata 4: "error: linking with `link.exe` failed"

```powershell
# MSVC redist kur
winget install Microsoft.VCRedist.2015+.x64
```

## Hata 5: "warning: unused variable"

Bu sadece uyarı, hata değil. Derleme devam eder. Düzeltmek için:
- Kodda değişken adının başına `_` ekle: `let _sev = ...`

## Hata 6: "thread 'main' has overflowed its stack"

```powershell
# Stack size artır
cargo build --release
# Release modda stack daha büyük
```

## Hata 7: "Access is denied"

```powershell
# PowerShell'i YÖNETİCİ olarak aç (Win+X → Terminal (Admin))
# VEYA antivirus'ü geçici kapat
```

---

# ═══════════════════════════════════════════════════════════════════════════════
#  BÖLÜM 3: ADIM ADIM KURULUM (SIFIRDAN)
# ═══════════════════════════════════════════════════════════════════════════════

## Adım 1: Gerekli Araçları Kur (10 dk)

```powershell
# PowerShell'i YÖNETİCİ olarak aç

# 1. Rust kur
winget install Rustlang.Rustup
# Yeni PowerShell aç
rustc --version && cargo --version

# 2. Git kur
winget install Git.Git

# 3. Visual Studio Build Tools kur
winget install Microsoft.VisualStudio.2022.BuildTools --override "--add Microsoft.VisualStudio.Workload.VCTools --passive"

# 4. FFmpeg kur (video/ses için)
# Scoop ile:
Set-ExecutionPolicy RemoteSigned -Scope CurrentUser
irm get.scoop.sh | iex
scoop install ffmpeg

# 5. Doğrula
rustc --version
cargo --version
git --version
ffmpeg -version
```

## Adım 2: Repoyu İndir (5 dk)

```powershell
cd C:\Users\AI_SYSTEM\Desktop
git clone https://github.com/nexsusagent-coder/SENTIENT_CORE.git
cd SENTIENT_CORE
```

## Adım 3: Derle (5-15 dk)

```powershell
# İlk derleme uzun sürer
cargo build --release

# BEKLENEN:
# Compiling sentient_core v4.0.0
# Compiling sentient_cli v4.0.0
# ...
# Finished release [optimized] target(s) in 5m 32s

# Binary:
dir .\target\release\sentient.exe
# BEKLENEN: ~20MB
```

## Adım 4: Test (5 dk)

```powershell
# Tüm testleri çalıştır
cargo test --workspace --lib 2>&1 | findstr "test result"

# BEKLENEN: "test result: ok. X passed; 0 failed"
# Windows'ta bazı testler farklı davranabilir (path, encoding)
```

## Adım 5: Çalıştır

```powershell
# Versiyon kontrol
.\target\release\sentient.exe --version

# Yardım
.\target\release\sentient.exe --help

# Sohbet başlat
.\target\release\sentient.exe chat
```

---

# ═══════════════════════════════════════════════════════════════════════════════
#  BÖLÜM 4: OLLAMA KURULUMU — LOKAL LLM
# ═══════════════════════════════════════════════════════════════════════════════

## Adım 1: Ollama Kur

```powershell
# https://ollama.com/download/windows
# İndir ve kur

# VEYA PowerShell ile:
winget install Ollama.Ollama
```

## Adım 2: Model İndir

```powershell
# Küçük model (4GB RAM)
ollama pull gemma2:2b

# Orta model (8GB RAM)
ollama pull qwen3:30b-a3b

# Büyük model (16GB RAM)
ollama pull deepseek-r1:8b

# Test
ollama run gemma2:2b
# "Merhaba, nasılsın?"
# /bye ile çık
```

## Adım 3: SENTIENT ile Bağla

```powershell
# .env oluştur
@"
OPENAI_API_BASE=http://localhost:11434/v1
OPENAI_API_KEY=ollama
DEFAULT_MODEL=ollama/gemma2:2b
"@ | Out-File -FilePath .env -Encoding utf8

# Sohbet başlat
.\target\release\sentient.exe chat
```

---

# ═══════════════════════════════════════════════════════════════════════════════
#  BÖLÜM 5: SESLİ ASİSTAN — WHISPER + PIPER
# ═══════════════════════════════════════════════════════════════════════════════

## Whisper.cpp Kur

```powershell
# Pre-built binary indir
wget https://github.com/ggerganov/whisper.cpp/releases/download/v1.7.4/whisper-1.7.4-bin-x64.zip
Expand-Archive whisper-1.7.4-bin-x64.zip -DestinationPath C:\whisper-cpp

# Model indir
mkdir C:\whisper-cpp\models
curl -L -o C:\whisper-cpp\models\ggml-medium.bin https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-medium.bin

# Test
C:\whisper-cpp\main.exe -m C:\whisper-cpp\models\ggml-medium.bin -f test.wav --language tr
```

## Piper TTS Kur

```powershell
# Binary indir
wget https://github.com/rhasspy/piper/releases/download/v1.2.0/piper_1.2.0-amd64.zip
Expand-Archive piper_1.2.0-amd64.zip -DestinationPath C:\piper

# Türkçe ses modeli
mkdir C:\piper\models
curl -L -o C:\piper\models\tr_TR-medium.onnx https://huggingface.co/rhasspy/piper-voices/resolve/main/tr/tr_TR/medium/tr_TR-medium.onnx
curl -L -o C:\piper\models\tr_TR-medium.onnx.json https://huggingface.co/rhasspy/piper-voices/resolve/main/tr/tr_TR/medium/tr_TR-medium.onnx.json

# Test
echo "Merhaba, ben Sentient" | C:\piper\piper.exe --model C:\piper\models\tr_TR-medium.onnx --output_file test.wav
ffplay test.wav
```

## .env Yapılandır

```powershell
@"
VOICE_ENABLED=true
VOICE_STT=whisper_cpp
VOICE_TTS=piper
VOICE_LANGUAGE=tr
WHISPER_MODEL_PATH=C:\whisper-cpp\models\ggml-medium.bin
PIPER_MODEL_PATH=C:\piper\models\tr_TR-medium.onnx
"@ | Out-File -FilePath .env -Encoding utf8 -Append
```

## JARVIS Başlat

```powershell
.\target\release\sentient.exe voice --wake-word "hey sentient" --language tr

# BEKLENEN:
# 🎤 Listening for wake word: "hey sentient"
```

---

# ═══════════════════════════════════════════════════════════════════════════════
#  BÖLÜM 6: SORUN GİDERME — HIZLI REFERANS
# ═══════════════════════════════════════════════════════════════════════════════

| Sorun | Çözüm |
|-------|-------|
| PyO3 hatası | `cargo clean && cargo build --release` (PyO3 devre dışı) |
| rustc not found | `winget install Rustlang.Rustup` |
| link.exe not found | VS Build Tools kur |
| Mikrofon çalışmıyor | Ayarlar → Gizlilik → Mikrofon → İzin ver |
| FFmpeg bulunamadı | `scoop install ffmpeg` |
| Ollama bağlanmıyor | `ollama serve` başlat |
| Uzun derleme | Normal, ilk derleme 10-15 dk |
| Antivirus engelliyor | Projeyi dışla veya geçici kapat |

---

# ═══════════════════════════════════════════════════════════════════════════════
#  BÖLÜM 7: SONRAKI ADIMLAR
# ═══════════════════════════════════════════════════════════════════════════════

Başarılı kurulumdan sonra:

1. `Arsiv/EVE_GIDINCE_YAPILACAKLAR_TEST_REHBERI.md` dosyasını takip et
2. Testleri çalıştır: `cargo test --workspace --lib`
3. .env dosyasına API key ekle
4. Ollama modeli indir
5. Sesli asistanı test et

---

*Windows Kurulum Rehberi v1.0*
*SENTIENT OS v4.0.0*
