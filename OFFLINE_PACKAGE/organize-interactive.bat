@echo off
setlocal enabledelayedexpansion

echo 📁 Folder Organizer - Interactive Launcher
echo ==========================================
echo.

REM Get the directory where this batch file is located
set "SCRIPT_DIR=%~dp0"

REM Look for the executable in common locations
set "EXE_PATH="

REM Check if we're in the project structure
if exist "!SCRIPT_DIR!src-tauri\target\release\folder-organizer.exe" (
    set "EXE_PATH=!SCRIPT_DIR!src-tauri\target\release\folder-organizer.exe"
) else if exist "!SCRIPT_DIR!folder-organizer.exe" (
    set "EXE_PATH=!SCRIPT_DIR!folder-organizer.exe"
) else if exist "!SCRIPT_DIR!..\src-tauri\target\release\folder-organizer.exe" (
    set "EXE_PATH=!SCRIPT_DIR!..\src-tauri\target\release\folder-organizer.exe"
) else (
    echo ❌ Error: folder-organizer.exe not found!
    echo.
    echo Please ensure the executable is in one of these locations:
    echo - Same folder as this batch file
    echo - src-tauri\target\release\ folder
    echo - Parent folder
    echo.
    pause
    exit /b 1
)

echo ✅ Found executable: !EXE_PATH!
echo.

:input_directory
set /p "directory=Enter folder path to organize: "
if "!directory!"=="" goto input_directory

echo.
echo Options:
echo 1. Normal organization
echo 2. Preview only (dry run)
echo 3. Include subdirectories (recursive)
echo 4. Undo last organization
echo 5. Preview + Recursive
echo.

set /p "choice=Choose option (1-5): "

if "!choice!"=="1" (
    echo 🚀 Running normal organization...
    "!EXE_PATH!" "!directory!"
) else if "!choice!"=="2" (
    echo 👁️ Running preview mode...
    "!EXE_PATH!" "!directory!" --dry-run
) else if "!choice!"=="3" (
    echo 🔄 Running recursive organization...
    "!EXE_PATH!" "!directory!" --recursive
) else if "!choice!"=="4" (
    echo ↩️ Undoing last organization...
    "!EXE_PATH!" "!directory!" --undo
) else if "!choice!"=="5" (
    echo 👁️ Running preview + recursive...
    "!EXE_PATH!" "!directory!" --dry-run --recursive
) else (
    echo ❌ Invalid choice!
    pause
    exit /b
)

echo.
echo ✅ Done! Press any key to exit...
pause >nul 