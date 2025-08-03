@echo off
echo 📁 Folder Organizer - Easy Launcher
echo ===================================
echo.

if "%1"=="" (
    echo Usage: organize.bat [directory] [options]
    echo.
    echo Examples:
    echo   organize.bat "C:\Users\uzair\Downloads"
    echo   organize.bat "C:\Users\uzair\Downloads" --dry-run
    echo   organize.bat "C:\Users\uzair\Downloads" --recursive
    echo   organize.bat "C:\Users\uzair\Downloads" --undo
    echo.
    pause
    exit /b
)

echo 🎯 Organizing: %1
echo.

cd /d "%~dp0src-tauri\target\release"
folder-organizer.exe %*

echo.
echo ✅ Done! Press any key to exit...
pause >nul 