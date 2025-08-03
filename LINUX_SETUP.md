# 🐧 Linux Setup Guide

## **📁 Folder Organizer for Linux**

This guide shows you how to build and use the folder organizer on Linux systems.

## **🚀 Quick Start (Ubuntu/Debian)**

### **1. Install Rust**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### **2. Build the Executable**
```bash
# Make build script executable
chmod +x build-linux.sh

# Run the build script
./build-linux.sh
```

### **3. Make Portable**
```bash
# Copy files to a portable location
cp target/release/folder-organizer ~/folder-organizer
cp folder-organizer.sh ~/folder-organizer.sh

# Make both executable
chmod +x ~/folder-organizer ~/folder-organizer.sh
```

### **4. Use the Tool**
```bash
# Run the interactive launcher
./folder-organizer.sh

# Or run directly
./folder-organizer "/path/to/folder" --dry-run
```

## **📋 Manual Build Steps**

### **1. Install Dependencies**
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install build-essential

# CentOS/RHEL/Fedora
sudo yum groupinstall "Development Tools"
# or
sudo dnf groupinstall "Development Tools"
```

### **2. Install Rust**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### **3. Build the Project**
```bash
# Navigate to the project directory
cd src-tauri

# Build the executable
cargo build --release
```

### **4. Make it Portable**
```bash
# Copy the executable
cp target/release/folder-organizer ~/folder-organizer

# Copy the shell script
cp ../folder-organizer.sh ~/folder-organizer.sh

# Make both executable
chmod +x ~/folder-organizer ~/folder-organizer.sh
```

## **🎯 Usage Examples**

### **Interactive Mode (Recommended)**
```bash
./folder-organizer.sh
```
Follow the prompts to:
1. Enter folder path
2. Choose operation type
3. See results

### **Command Line Mode**
```bash
# Preview changes
./folder-organizer "/home/user/Downloads" --dry-run

# Organize files
./folder-organizer "/home/user/Downloads"

# Include subdirectories
./folder-organizer "/home/user/Downloads" --recursive

# Undo last organization
./folder-organizer "/home/user/Downloads" --undo
```

## **📁 File Structure**
```
folder-organizer/
├── folder-organizer          # Linux executable
├── folder-organizer.sh       # Interactive launcher
├── build-linux.sh           # Build script
└── src-tauri/
    ├── src/
    │   ├── main.rs          # Main program
    │   ├── organizer.rs      # Core logic
    │   └── error.rs         # Error handling
    └── Cargo.toml           # Dependencies
```

## **🔧 Troubleshooting**

### **"Permission denied" error**
```bash
chmod +x folder-organizer folder-organizer.sh
```

### **"Command not found: cargo"**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### **Build fails with linker errors**
```bash
# Install build tools
sudo apt install build-essential  # Ubuntu/Debian
sudo yum groupinstall "Development Tools"  # CentOS/RHEL
```

## **🚀 Advanced Usage**

### **Create a Desktop Shortcut**
```bash
# Create desktop entry
cat > ~/.local/share/applications/folder-organizer.desktop << EOF
[Desktop Entry]
Name=Folder Organizer
Comment=Organize files into categories
Exec=/path/to/folder-organizer.sh
Terminal=true
Type=Application
Categories=Utility;
EOF
```

### **Add to PATH**
```bash
# Add to your shell profile
echo 'export PATH="$HOME:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## **📊 Features**

- ✅ **Cross-platform** - Works on any Linux distribution
- ✅ **No dependencies** - Single executable
- ✅ **Interactive mode** - Easy-to-use menu
- ✅ **Command line** - Scriptable and powerful
- ✅ **Preview mode** - Safe dry-run option
- ✅ **Undo functionality** - Revert changes
- ✅ **Recursive processing** - Include subdirectories

## **🎉 Success!**

Your folder organizer is now ready to use on Linux! The tool will:
- Organize files into categories (Images, Videos, Documents, etc.)
- Show progress and statistics
- Create log files for undo operations
- Work on any Linux system without installation 