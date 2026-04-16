#!/bin/bash
# ═══════════════════════════════════════════════════════════════════════════════
#  SENTIENT OS v4.0.0 — FULL HEALTH CHECK
#  Eve gelince bu scripti çalıştır: ./scripts/sentient-health-check.sh
# ═══════════════════════════════════════════════════════════════════════════════

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m'

PASS=0
FAIL=0
WARN=0

check() {
    local name="$1"
    local result="$2"
    if [ "$result" = "pass" ]; then
        echo -e "  ${GREEN}✅ $name${NC}"
        PASS=$((PASS + 1))
    elif [ "$result" = "fail" ]; then
        echo -e "  ${RED}❌ $name${NC}"
        FAIL=$((FAIL + 1))
    else
        echo -e "  ${YELLOW}⚠️  $name${NC}"
        WARN=$((WARN + 1))
    fi
}

echo ""
echo -e "${CYAN}═════════════════════════════════════════════════════════${NC}"
echo -e "${BOLD}  🐺 SENTIENT OS v4.0.0 — FULL HEALTH CHECK${NC}"
echo -e "${CYAN}═════════════════════════════════════════════════════════${NC}"
echo -e "  Tarih: $(date '+%Y-%m-%d %H:%M:%S')"
echo ""

# ─── 1. BINARY ─────────────────────────────────────────────────────────────
echo -e "${BOLD}📦 Binary${NC}"
if [ -f "./target/release/sentient" ]; then
    V=$(./target/release/sentient --version 2>/dev/null || echo "unknown")
    check "sentient binary: $V" "pass"
else
    check "sentient binary: NOT FOUND (run: cargo build --release)" "fail"
fi

# ─── 2. RUST ────────────────────────────────────────────────────────────────
echo -e "${BOLD}🦀 Rust Toolchain${NC}"
if command -v rustc &>/dev/null; then
    RV=$(rustc --version 2>/dev/null)
    check "rustc: $RV" "pass"
else
    check "rustc: NOT FOUND" "fail"
fi

# ─── 3. TESTS ───────────────────────────────────────────────────────────────
echo -e "${BOLD}🧪 Tests${NC}"
echo -e "  ${YELLOW}⏳ Running workspace tests (may take 2-5 minutes)...${NC}"
TEST_OUTPUT=$(cargo test --workspace --lib 2>&1 || true)
TOTAL_PASSED=$(echo "$TEST_OUTPUT" | grep -oP '\d+(?= passed)' | awk '{sum+=$1} END{print sum+0}')
TOTAL_FAILED=$(echo "$TEST_OUTPUT" | grep -oP '\d+(?= failed)' | awk '{sum+=$1} END{print sum+0}')
if [ "$TOTAL_FAILED" = "0" ] || [ -z "$TOTAL_FAILED" ]; then
    check "Tests: ${TOTAL_PASSED} passed, 0 failed" "pass"
else
    check "Tests: ${TOTAL_PASSED} passed, ${TOTAL_FAILED} FAILED" "fail"
fi

# ─── 4. OLLAMA ───────────────────────────────────────────────────────────────
echo -e "${BOLD}🦙 Ollama${NC}"
if curl -s --max-time 3 http://localhost:11434/api/tags &>/dev/null; then
    MODELS=$(curl -s http://localhost:11434/api/tags | python3 -c "import sys,json; print(len(json.load(sys.stdin).get('models',[])))" 2>/dev/null || echo "?")
    check "Ollama: Running ($MODELS models)" "pass"
else
    check "Ollama: Not running (start: ollama serve)" "fail"
fi

# ─── 5. QDRANT ─────────────────────────────────────────────────────────────
echo -e "${BOLD}🔍 Qdrant${NC}"
if curl -s --max-time 3 http://localhost:6333/collections &>/dev/null; then
    check "Qdrant: Running (port 6333)" "pass"
else
    check "Qdrant: Not running (start: docker-compose up -d qdrant)" "warn"
fi

# ─── 6. GATEWAY ─────────────────────────────────────────────────────────────
echo -e "${BOLD}🌐 Gateway${NC}"
if curl -s --max-time 3 http://localhost:8080/ &>/dev/null; then
    check "Gateway: Running (http://localhost:8080/dashboard)" "pass"
else
    check "Gateway: Not running (start: ./target/release/sentient-web &)" "warn"
fi

# ─── 7. DOCKER ──────────────────────────────────────────────────────────────
echo -e "${BOLD}🐳 Docker${NC}"
if docker ps &>/dev/null; then
    CONTAINERS=$(docker ps --format "{{.Names}}" | wc -l)
    check "Docker: Running ($CONTAINERS containers)" "pass"
else
    check "Docker: Not running (start: sudo systemctl start docker)" "warn"
fi

# ─── 8. GPU ──────────────────────────────────────────────────────────────────
echo -e "${BOLD}🎮 GPU${NC}"
if command -v nvidia-smi &>/dev/null; then
    GPU_NAME=$(nvidia-smi --query-gpu=name --format=csv,noheader 2>/dev/null | head -1)
    GPU_VRAM=$(nvidia-smi --query-gpu=memory.total --format=csv,noheader 2>/dev/null | head -1)
    GPU_USED=$(nvidia-smi --query-gpu=memory.used --format=csv,noheader 2>/dev/null | head -1)
    check "GPU: $GPU_NAME ($GPU_VRAM, $GPU_USED used)" "pass"
elif system_profiler SPAudioDataType &>/dev/null 2>&1; then
    CHIP=$(system_profiler SPDisplaysDataType 2>/dev/null | grep "Chipset" | head -1 | awk -F': ' '{print $2}')
    if [ -n "$CHIP" ]; then
        check "GPU (Apple): $CHIP" "pass"
    else
        check "GPU: Not detected" "warn"
    fi
else
    check "GPU: Not detected (LLM will use CPU, slower)" "warn"
fi

# ─── 9. MICROPHONE ──────────────────────────────────────────────────────────
echo -e "${BOLD}🎤 Microphone${NC}"
if arecord -l 2>/dev/null | grep -q "card"; then
    CARD=$(arecord -l 2>/dev/null | grep "card" | head -1)
    check "Microphone: $CARD" "pass"
elif system_profiler SPAudioDataType 2>/dev/null | grep -q "Input"; then
    check "Microphone: Detected (macOS)" "pass"
else
    check "Microphone: Not detected (JARVIS won't work)" "warn"
fi

# ─── 10. DISPLAY ────────────────────────────────────────────────────────────
echo -e "${BOLD}🖥️ Display${NC}"
if [ -n "$DISPLAY" ]; then
    check "Display: $DISPLAY" "pass"
elif [ -n "$WAYLAND_DISPLAY" ]; then
    check "Display: Wayland ($WAYLAND_DISPLAY)" "pass"
else
    check "Display: Not set (desktop automation won't work)" "warn"
fi

# ─── 11. .ENV ───────────────────────────────────────────────────────────────
echo -e "${BOLD}⚙️ Configuration${NC}"
if [ -f ".env" ]; then
    KEY_COUNT=$(grep -c "KEY\|TOKEN\|SECRET\|PASSWORD" .env 2>/dev/null || echo 0)
    check ".env: Found ($KEY_COUNT credentials)" "pass"
else
    check ".env: Missing (run: cp .env.example .env)" "fail"
fi

# ─── 12. API KEYS ───────────────────────────────────────────────────────────
echo -e "${BOLD}🔑 API Keys${NC}"
if grep -q "OPENROUTER_API_KEY=sk" .env 2>/dev/null; then
    check "OpenRouter API key: Found" "pass"
else
    check "OpenRouter API key: Not found" "warn"
fi
if grep -q "OPENAI_API_KEY=sk" .env 2>/dev/null; then
    check "OpenAI API key: Found" "pass"
else
    check "OpenAI API key: Not found" "warn"
fi
if grep -q "ANTHROPIC_API_KEY=sk" .env 2>/dev/null; then
    check "Anthropic API key: Found" "pass"
else
    check "Anthropic API key: Not found" "warn"
fi

# ─── 13. BROWSER ────────────────────────────────────────────────────────────
echo -e "${BOLD}🌐 Browser${NC}"
if command -v firefox &>/dev/null; then
    check "Firefox: $(firefox --version 2>/dev/null | head -1)" "pass"
elif command -v chromium &>/dev/null || command -v google-chrome &>/dev/null; then
    check "Chromium/Chrome: Found" "pass"
else
    check "Browser: Not found (browser automation won't work)" "warn"
fi

# ─── 14. WHISPER ────────────────────────────────────────────────────────────
echo -e "${BOLD}🗣️ Speech${NC}"
if command -v whisper &>/dev/null || [ -f "$HOME/whisper.cpp/main" ]; then
    check "Whisper (STT): Found" "pass"
else
    check "Whisper (STT): Not found (JARVIS won't understand you)" "warn"
fi
if command -v piper &>/dev/null || [ -f "$HOME/.local/bin/piper" ]; then
    check "Piper (TTS): Found" "pass"
else
    check "Piper (TTS): Not found (JARVIS won't speak)" "warn"
fi

# ─── SUMMARY ────────────────────────────────────────────────────────────────
echo ""
echo -e "${CYAN}═════════════════════════════════════════════════════════${NC}"
echo -e "${BOLD}  SONUÇ${NC}"
echo -e "${CYAN}═════════════════════════════════════════════════════════${NC}"
echo -e "  ${GREEN}✅ Geçen:  $PASS${NC}"
echo -e "  ${RED}❌ Başarısız: $FAIL${NC}"
echo -e "  ${YELLOW}⚠️  Uyarı: $WARN${NC}"
echo ""

TOTAL=$((PASS + FAIL + WARN))
if [ "$FAIL" -eq 0 ]; then
    echo -e "  ${GREEN}${BOLD}🎉 SİSTEM HAZIR! Tüm kritik kontroller geçti.${NC}"
    echo ""
    echo -e "  Sonraki adım: ${CYAN}./Arsiv/EVE_GIDINCE_YAPILACAKLAR_TEST_REHBERI.md${NC}"
else
    echo -e "  ${RED}${BOLD}⚠️  $FAIL kritik sorun var! Yukarıdaki ❌'leri çöz.${NC}"
    echo ""
    echo -e "  Yardım: ${CYAN}./Arsiv/EVE_GIDINCE_YAPILACAKLAR_TEST_REHBERI.md → Bölüm 22${NC}"
fi
echo ""
