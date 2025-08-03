@echo off
setlocal enabledelayedexpansion

echo 📁 Folder Organizer - Portable Launcher
echo =======================================
echo.

REM Check if executable exists in the same folder
if not exist "folder-organizer-portable.exe" (
    echo ❌ Error: folder-organizer-portable.exe not found in this folder!
    echo.
    echo To make this portable:
    echo 1. Copy folder-organizer-portable.exe to the same folder as this batch file
    echo 2. Run this batch file again
    echo.
    pause
    exit /b 1
)

echo ✅ Found executable: folder-organizer-portable.exe
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
    folder-organizer-portable.exe "!directory!"
) else if "!choice!"=="2" (
    echo 👁️ Running preview mode...
    folder-organizer-portable.exe "!directory!" --dry-run
) else if "!choice!"=="3" (
    echo 🔄 Running recursive organization...
    folder-organizer-portable.exe "!directory!" --recursive
) else if "!choice!"=="4" (
    echo ↩️ Undoing last organization...
    folder-organizer-portable.exe "!directory!" --undo
) else if "!choice!"=="5" (
    echo 👁️ Running preview + recursive...
    folder-organizer-portable.exe "!directory!" --dry-run --recursive
) else (
    echo ❌ Invalid choice!
    pause
    exit /b
)

echo.
echo ✅ Done! Press any key to exit...
pause >nul 