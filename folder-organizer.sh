#!/bin/bash

echo "📁 Folder Organizer - Linux Launcher"
echo "====================================="
echo

# Check if executable exists
if [ ! -f "./folder-organizer" ]; then
    echo "❌ Error: folder-organizer executable not found in this folder!"
    echo
    echo "To make this portable:"
    echo "1. Copy folder-organizer to the same folder as this script"
    echo "2. Make it executable: chmod +x folder-organizer"
    echo "3. Run this script again"
    echo
    read -p "Press Enter to exit..."
    exit 1
fi

echo "✅ Found executable: folder-organizer"
echo

# Get directory input
read -p "Enter folder path to organize: " directory
while [ -z "$directory" ]; do
    read -p "Enter folder path to organize: " directory
done

echo
echo "Options:"
echo "1. Normal organization"
echo "2. Preview only (dry run)"
echo "3. Include subdirectories (recursive)"
echo "4. Undo last organization"
echo "5. Preview + Recursive"
echo

read -p "Choose option (1-5): " choice

case $choice in
    1)
        echo "🚀 Running normal organization..."
        ./folder-organizer "$directory"
        ;;
    2)
        echo "👁️ Running preview mode..."
        ./folder-organizer "$directory" --dry-run
        ;;
    3)
        echo "🔄 Running recursive organization..."
        ./folder-organizer "$directory" --recursive
        ;;
    4)
        echo "↩️ Undoing last organization..."
        ./folder-organizer "$directory" --undo
        ;;
    5)
        echo "👁️ Running preview + recursive..."
        ./folder-organizer "$directory" --dry-run --recursive
        ;;
    *)
        echo "❌ Invalid choice!"
        read -p "Press Enter to exit..."
        exit 1
        ;;
esac

echo
echo "✅ Done! Press Enter to exit..."
read 