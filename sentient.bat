@echo off
REM ╔════════════════════════════════════════════════════════════════════════════════╗
REM ║  SENTIENT NEXUS OS - Global Launcher v7.0.0                                   ║
REM ║  Professional AI Agent Framework                                              ║
REM ╚════════════════════════════════════════════════════════════════════════════════╝

setlocal enabledelayedexpansion

:: Colors (Windows 10+)
for /f %%i in ('echo prompt $E^| cmd') do set "ESC=%%i"
set "RED=!ESC![31m"
set "GREEN=!ESC![32m"
set "CYAN=!ESC![36m"
set "YELLOW=!ESC![33m"
set "NC=!ESC![0m"

:: Find installation directory
set "SENTIENT_DIR=%SENTIENT_HOME%"
if not defined SENTIENT_DIR set "SENTIENT_DIR=%USERPROFILE%\.sentient"

set "INSTALL_DIR="
if exist "C:\opt\sentient" (
    set "INSTALL_DIR=C:\opt\sentient"
) else if exist "%SENTIENT_DIR%" (
    set "INSTALL_DIR=%SENTIENT_DIR%"
) else if exist "%~dp0Cargo.toml" (
    set "INSTALL_DIR=%~dp0"
)

if not defined INSTALL_DIR (
    echo %RED%ERROR: SENTIENT installation not found.%NC%
    echo Please run setup.ps1 first to install SENTIENT.
    exit /b 1
)

cd /d "%INSTALL_DIR%"

:: Parse arguments
set "MODE=%1"
if not defined MODE set "MODE=dashboard"

:: Help function
if "%MODE%"=="help" goto :show_help
if "%MODE%"=="--help" goto :show_help
if "%MODE%"=="-h" goto :show_help

:: Main execution
if "%MODE%"=="dashboard" goto :dashboard
if "%MODE%"=="ui" goto :dashboard
if "%MODE%"=="" goto :dashboard

if "%MODE%"=="shell" goto :shell
if "%MODE%"=="cli" goto :shell
if "%MODE%"=="repl" goto :shell

if "%MODE%"=="setup" goto :setup
if "%MODE%"=="configure" goto :setup
if "%MODE%"=="init" goto :setup

if "%MODE%"=="config" goto :config
if "%MODE%"=="status" goto :status
if "%MODE%"=="update" goto :update
if "%MODE%"=="logs" goto :logs

echo %RED%Unknown command: %MODE%%NC%
echo.
goto :show_help

:show_help
echo %CYAN%SENTIENT NEXUS OS - Global Launcher%NC%
echo.
echo Usage: sentient [command]
echo.
echo Commands:
echo   dashboard, ui    Launch the web dashboard (default)
echo   shell, cli       Launch the interactive CLI
echo   setup            Run the setup wizard
echo   config           Open configuration
echo   status           Show system status
echo   update           Update to latest version
echo   logs             View system logs
echo   help, --help     Show this help message
echo.
echo Examples:
echo   sentient              # Launch dashboard
echo   sentient shell        # Launch CLI
echo   sentient setup        # Reconfigure
echo.
exit /b 0

:dashboard
echo %CYAN%Starting SENTIENT Dashboard...%NC%
echo.

if exist "%INSTALL_DIR%\target\release\sentient-dashboard.exe" (
    "%INSTALL_DIR%\target\release\sentient-dashboard.exe"
) else if exist "%INSTALL_DIR%\target\debug\sentient-dashboard.exe" (
    "%INSTALL_DIR%\target\debug\sentient-dashboard.exe"
) else if exist "%INSTALL_DIR%\dashboard\package.json" (
    cd /d "%INSTALL_DIR%\dashboard"
    where npm >nul 2>&1
    if !errorlevel!==0 (
        npm run dev 2>nul || npm start
    ) else (
        echo %RED%ERROR: npm not found. Please install Node.js.%NC%
        exit /b 1
    )
) else if exist "%INSTALL_DIR%\start-dashboard.bat" (
    call "%INSTALL_DIR%\start-dashboard.bat"
) else (
    echo %RED%ERROR: No dashboard binary found.%NC%
    echo Please build the project first: cargo build --release
    exit /b 1
)
exit /b 0

:shell
echo %CYAN%Starting SENTIENT Shell...%NC%
echo.

if exist "%INSTALL_DIR%\target\release\sentient-shell.exe" (
    "%INSTALL_DIR%\target\release\sentient-shell.exe"
) else if exist "%INSTALL_DIR%\target\debug\sentient-shell.exe" (
    "%INSTALL_DIR%\target\debug\sentient-shell.exe"
) else (
    echo %RED%ERROR: sentient-shell binary not found.%NC%
    echo Please build the project first: cargo build --release
    exit /b 1
)
exit /b 0

:setup
echo %CYAN%Starting SENTIENT Setup Wizard...%NC%
echo.

if exist "%INSTALL_DIR%\target\release\sentient-setup.exe" (
    "%INSTALL_DIR%\target\release\sentient-setup.exe"
) else if exist "%INSTALL_DIR%\target\debug\sentient-setup.exe" (
    "%INSTALL_DIR%\target\debug\sentient-setup.exe"
) else if exist "%INSTALL_DIR%\setup.ps1" (
    powershell -ExecutionPolicy Bypass -File "%INSTALL_DIR%\setup.ps1"
) else (
    echo %RED%ERROR: Setup wizard not found.%NC%
    exit /b 1
)
exit /b 0

:config
set "CONFIG_FILE=%APPDATA%\sentient\config.toml"
if exist "%CONFIG_FILE%" (
    if defined EDITOR (
        %EDITOR% "%CONFIG_FILE%"
    ) else (
        notepad "%CONFIG_FILE%"
    )
) else (
    echo %YELLOW%Config file not found. Run 'sentient setup' first.%NC%
)
exit /b 0

:status
echo %CYAN%SENTIENT System Status%NC%
echo.
echo Installation: %INSTALL_DIR%

:: Check if dashboard is running
tasklist /FI "IMAGENAME eq sentient-dashboard.exe" 2>NUL | find /I /N "sentient-dashboard.exe">NUL
if "%ERRORLEVEL%"=="0" (
    echo Dashboard: %GREEN%Running%NC%
) else (
    echo Dashboard: %YELLOW%Not running%NC%
)

:: Check config
if exist "%APPDATA%\sentient\config.toml" (
    echo Config: %GREEN%Configured%NC%
) else (
    echo Config: %YELLOW%Not configured (run 'sentient setup')%NC%
)

:: Check Ollama
where ollama >nul 2>&1
if %errorlevel%==0 (
    echo Ollama: %GREEN%Installed%NC%
) else (
    echo Ollama: %YELLOW%Not installed%NC%
)
echo.
exit /b 0

:update
echo %CYAN%Updating SENTIENT...%NC%
cd /d "%INSTALL_DIR%"
if exist ".git" (
    git pull origin main
    cargo build --release
    echo %GREEN%Update complete!%NC%
) else (
    echo %YELLOW%Not a git repository. Manual update required.%NC%
)
exit /b 0

:logs
set "LOG_DIR=%LOCALAPPDATA%\sentient\logs"
if exist "%LOG_DIR%" (
    dir "%LOG_DIR%"
    echo.
    set /p "LOGFILE=Enter log file to view: "
    if exist "%LOG_DIR%\!LOGFILE!" (
        type "%LOG_DIR%\!LOGFILE!"
    )
) else (
    echo %YELLOW%No logs found.%NC%
)
exit /b 0
