@echo off
echo ========================================
echo    Creating Offline Package
echo ========================================
echo.

echo Step 1: Creating offline package directory...
if exist "OFFLINE_PACKAGE" rmdir /s /q "OFFLINE_PACKAGE"
mkdir "OFFLINE_PACKAGE"

echo Step 2: Copying executable files...
copy "folder-organizer-portable.exe" "OFFLINE_PACKAGE\"
copy "organize-interactive.bat" "OFFLINE_PACKAGE\"
copy "folder-organizer-portable.bat" "OFFLINE_PACKAGE\"

echo Step 3: Copying Linux files...
copy "folder-organizer.sh" "OFFLINE_PACKAGE\"
copy "build-linux.sh" "OFFLINE_PACKAGE\"

echo Step 4: Copying documentation...
copy "README.md" "OFFLINE_PACKAGE\"
copy "LINUX_SETUP.md" "OFFLINE_PACKAGE\"
copy "OFFLINE_PACKAGE\README.md" "OFFLINE_PACKAGE\"

echo Step 5: Creating source code package...
mkdir "OFFLINE_PACKAGE\source"
copy "Cargo.toml" "OFFLINE_PACKAGE\source\"
copy "src\*.rs" "OFFLINE_PACKAGE\source\"

echo Step 6: Creating usage examples...
echo @echo off > "OFFLINE_PACKAGE\example-usage.bat"
echo echo Folder Organizer - Example Usage >> "OFFLINE_PACKAGE\example-usage.bat"
echo echo ================================ >> "OFFLINE_PACKAGE\example-usage.bat"
echo echo. >> "OFFLINE_PACKAGE\example-usage.bat"
echo echo 1. Interactive mode: >> "OFFLINE_PACKAGE\example-usage.bat"
echo echo    organize-interactive.bat >> "OFFLINE_PACKAGE\example-usage.bat"
echo echo. >> "OFFLINE_PACKAGE\example-usage.bat"
echo echo 2. Direct usage: >> "OFFLINE_PACKAGE\example-usage.bat"
echo echo    folder-organizer-portable.exe "C:\Users\YourName\Downloads" >> "OFFLINE_PACKAGE\example-usage.bat"
echo echo. >> "OFFLINE_PACKAGE\example-usage.bat"
echo echo 3. Preview changes: >> "OFFLINE_PACKAGE\example-usage.bat"
echo echo    folder-organizer-portable.exe "C:\Users\YourName\Downloads" --dry-run >> "OFFLINE_PACKAGE\example-usage.bat"
echo echo. >> "OFFLINE_PACKAGE\example-usage.bat"
echo echo 4. Undo changes: >> "OFFLINE_PACKAGE\example-usage.bat"
echo echo    folder-organizer-portable.exe "C:\Users\YourName\Downloads" --undo >> "OFFLINE_PACKAGE\example-usage.bat"
echo echo. >> "OFFLINE_PACKAGE\example-usage.bat"
echo pause >> "OFFLINE_PACKAGE\example-usage.bat"

echo Step 7: Creating safety information...
echo # Safety Information > "OFFLINE_PACKAGE\SAFETY.md"
echo. >> "OFFLINE_PACKAGE\SAFETY.md"
echo ## Why This Tool is Safe >> "OFFLINE_PACKAGE\SAFETY.md"
echo. >> "OFFLINE_PACKAGE\SAFETY.md"
echo - **No internet connection required** >> "OFFLINE_PACKAGE\SAFETY.md"
echo - **No data sent anywhere** >> "OFFLINE_PACKAGE\SAFETY.md"
echo - **No account creation needed** >> "OFFLINE_PACKAGE\SAFETY.md"
echo - **No personal information collected** >> "OFFLINE_PACKAGE\SAFETY.md"
echo - **Works completely offline** >> "OFFLINE_PACKAGE\SAFETY.md"
echo - **Source code included for verification** >> "OFFLINE_PACKAGE\SAFETY.md"
echo. >> "OFFLINE_PACKAGE\SAFETY.md"
echo ## Safety Features >> "OFFLINE_PACKAGE\SAFETY.md"
echo. >> "OFFLINE_PACKAGE\SAFETY.md"
echo - **Dry-run mode**: Preview changes without moving files >> "OFFLINE_PACKAGE\SAFETY.md"
echo - **Undo functionality**: Restore files to original locations >> "OFFLINE_PACKAGE\SAFETY.md"
echo - **Detailed logging**: Every action is recorded >> "OFFLINE_PACKAGE\SAFETY.md"
echo - **No file deletion**: Only moves files, never deletes >> "OFFLINE_PACKAGE\SAFETY.md"

echo.
echo ========================================
echo    Package Created Successfully!
echo ========================================
echo.
echo The offline package is ready in: OFFLINE_PACKAGE\
echo.
echo Contents:
echo - folder-organizer-portable.exe (Windows executable)
echo - organize-interactive.bat (Easy menu for Windows)
echo - folder-organizer.sh (Linux script)
echo - build-linux.sh (Linux build script)
echo - README.md (Usage instructions)
echo - SAFETY.md (Safety information)
echo - source\ (Source code for verification)
echo - example-usage.bat (Usage examples)
echo.
echo You can now share this OFFLINE_PACKAGE folder with your friends!
echo They don't need GitHub access or any accounts.
echo.
pause 