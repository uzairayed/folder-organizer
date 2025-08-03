# 📁 Folder Organizer

A powerful, cross-platform command-line tool to automatically organize files into categorized folders based on their file extensions.

## ✨ Features

- **🎯 Smart Categorization** - Automatically sorts files into 9 categories
- **🔄 Recursive Processing** - Include subdirectories in organization
- **👁️ Preview Mode** - Safe dry-run to preview changes before applying
- **↩️ Undo Functionality** - Revert the last organization operation
- **📊 Detailed Logging** - Track all file movements for safety
- **🚀 Cross-Platform** - Works on Windows, Linux, and macOS
- **💻 Interactive Mode** - Easy-to-use menu system
- **⚡ High Performance** - Fast processing with Rust backend

## 📋 File Categories

| Category | File Extensions |
|----------|----------------|
| 📷 **Images** | jpg, jpeg, png, gif, bmp, tiff, webp, svg |
| 🎥 **Videos** | mp4, avi, mov, wmv, flv, mkv, webm, m4v |
| 📄 **Documents** | pdf, doc, docx, txt, rtf, odt, pages |
| 🎵 **Audio** | mp3, wav, flac, aac, ogg, wma, m4a |
| 📦 **Archives** | zip, rar, 7z, tar, gz, bz2, xz |
| 💻 **Code** | py, js, ts, java, cpp, c, h, rs, go, php, html, css, xml, json, yaml, toml, md |
| 📊 **Spreadsheets** | xls, xlsx, csv, ods, numbers |
| 📋 **Presentations** | ppt, pptx, key, odp |
| 📁 **Others** | Uncategorized files |

## 🚀 Quick Start

### Windows

1. **Download the portable version:**
   - `folder-organizer-portable.exe`
   - `folder-organizer-portable.bat`

2. **Run the interactive launcher:**
   ```cmd
   folder-organizer-portable.bat
   ```

3. **Or use command line:**
   ```cmd
   folder-organizer-portable.exe "C:\Users\username\Downloads" --dry-run
   ```

### Linux

1. **Build from source:**
   ```bash
   chmod +x build-linux.sh
   ./build-linux.sh
   ```

2. **Run the interactive launcher:**
   ```bash
   ./folder-organizer.sh
   ```

3. **Or use command line:**
   ```bash
   ./folder-organizer "/home/user/Downloads" --dry-run
   ```

## 📖 Usage Examples

### Basic Organization
```bash
# Organize files in Downloads folder
folder-organizer "C:\Users\username\Downloads"

# Preview changes first (recommended)
folder-organizer "C:\Users\username\Downloads" --dry-run
```

### Advanced Options
```bash
# Include subdirectories
folder-organizer "C:\Users\username\Downloads" --recursive

# Preview with subdirectories
folder-organizer "C:\Users\username\Downloads" --dry-run --recursive

# Undo last organization
folder-organizer "C:\Users\username\Downloads" --undo
```

### Interactive Mode
```bash
# Windows
folder-organizer-portable.bat

# Linux
./folder-organizer.sh
```

## 🛠️ Development

### Prerequisites
- [Rust](https://rustup.rs/) (for building from source)
- [Node.js](https://nodejs.org/) (for frontend development)

### Building from Source

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/folder-organizer.git
   cd folder-organizer
   ```

2. **Build the executable:**
   ```bash
   cd src-tauri
   cargo build --release
   ```

3. **Create portable version:**
   ```bash
   cp target/release/folder-organizer.exe ../folder-organizer-portable.exe
   ```

### Project Structure
```
folder-organizer/
├── src-tauri/                 # Rust backend
│   ├── src/
│   │   ├── main.rs           # CLI interface
│   │   ├── organizer.rs      # Core logic
│   │   └── error.rs          # Error handling
│   └── Cargo.toml            # Dependencies
├── src/                       # Frontend (Vue.js)
├── folder-organizer-portable.exe  # Windows executable
├── folder-organizer-portable.bat  # Windows launcher
├── folder-organizer.sh        # Linux launcher
├── build-linux.sh            # Linux build script
└── LINUX_SETUP.md           # Linux setup guide
```

## 🔧 Configuration

### Custom Categories
Edit `src-tauri/src/organizer.rs` to modify file categories:

```rust
fn get_default_categories() -> HashMap<String, Vec<String>> {
    HashMap::from([
        ("Custom Category".to_string(), vec!["ext1".to_string(), "ext2".to_string()]),
        // ... existing categories
    ])
}
```

### Log Files
- Location: `.folder_organizer_log.json` in the target directory
- Contains: All file movements for undo operations
- Format: JSON with timestamps and file paths

## 🐛 Troubleshooting

### Common Issues

**"Permission denied" (Linux)**
```bash
chmod +x folder-organizer folder-organizer.sh
```

**"Executable not found"**
- Ensure the executable is in the same folder as the launcher
- Check file permissions (Linux)

**"Directory not found"**
- Use absolute paths: `C:\Users\username\Downloads`
- Ensure the directory exists and is accessible

### Getting Help
1. Run with `--dry-run` first to preview changes
2. Check the log file for detailed information
3. Use the undo feature if something goes wrong

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Rust](https://rust-lang.org/) for performance and safety
- Cross-platform compatibility with [Tauri](https://tauri.studio/)
- Modern UI with [Vue.js](https://vuejs.org/) and [Tailwind CSS](https://tailwindcss.com/)

## 📊 Statistics

- **Languages**: Rust, JavaScript, HTML, CSS
- **Platforms**: Windows, Linux, macOS
- **Categories**: 9 file types supported
- **Extensions**: 50+ file extensions recognized
- **Performance**: Fast processing with Rust backend

---

**⭐ Star this repository if you find it useful!**

**🐛 Report issues or suggest features in the Issues tab.** 
