@echo off
REM ═══════════════════════════════════════════════════════════════════════════════
REM  SENTIENT Dashboard Launcher
REM ═══════════════════════════════════════════════════════════════════════════════

echo.
echo ╔══════════════════════════════════════════════════════════════════════════════╗
echo ║                    SENTIENT DASHBOARD BASLATILIYOR                          ║
echo ╚══════════════════════════════════════════════════════════════════════════════╝
echo.

cd /d "%USERPROFILE%\sentient"

REM Config dosyasını kontrol et
if not exist "%USERPROFILE%\.sentient\config.toml" (
    echo [!] Config dosyasi bulunamadi!
    echo     Once setup calistirin: sentient-setup.exe
    echo.
    pause
    exit /b 1
)

REM Dashboard binary'sini kontrol et
if not exist "target\release\nexus-dashboard.exe" (
    echo [!] Derleme yapiliyor...
    cargo build --release --bin nexus-dashboard
)

echo [*] Dashboard http://localhost:8080 adresinde baslatiliyor...
echo [*] Durdurmak icin Ctrl+C basin
echo.

target\release\nexus-dashboard.exe --port 8080 --host 127.0.0.1

pause
