#!/bin/bash

echo "🐧 Building Folder Organizer for Linux"
echo "======================================"
echo

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust is not installed!"
    echo
    echo "To install Rust on Linux:"
    echo "1. Visit https://rustup.rs/"
    echo "2. Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "3. Restart your terminal"
    echo "4. Run this script again"
    echo
    exit 1
fi

echo "✅ Rust found: $(cargo --version)"
echo

# Build the executable
echo "🔨 Building folder-organizer..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo
    echo "📁 Executable location: target/release/folder-organizer"
    echo
    echo "🚀 To make it portable:"
    echo "1. Copy target/release/folder-organizer to any folder"
    echo "2. Copy folder-organizer.sh to the same folder"
    echo "3. Make both executable: chmod +x folder-organizer folder-organizer.sh"
    echo "4. Run: ./folder-organizer.sh"
    echo
else
    echo "❌ Build failed!"
    exit 1
fi 