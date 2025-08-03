mod organizer;
mod error;

use organizer::FileOrganizer;
use std::path::PathBuf;
use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("📁 Folder Organizer - Command Line Version");
        println!("==========================================");
        println!("Usage:");
        println!("  folder-organizer <directory> [options]");
        println!();
        println!("Options:");
        println!("  --recursive    Include subdirectories");
        println!("  --dry-run      Preview changes without moving files");
        println!("  --undo         Undo last organization");
        println!();
        println!("Examples:");
        println!("  folder-organizer C:\\Users\\uzair\\Downloads");
        println!("  folder-organizer C:\\Users\\uzair\\Downloads --recursive");
        println!("  folder-organizer C:\\Users\\uzair\\Downloads --dry-run");
        println!("  folder-organizer C:\\Users\\uzair\\Downloads --undo");
        println!();
        println!("💡 Tip: Double-click organize-interactive.bat for an easy menu!");
        println!();
        println!("Press Enter to exit...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        return;
    }

    let target_dir = PathBuf::from(&args[1]);
    let recursive = args.contains(&"--recursive".to_string());
    let dry_run = args.contains(&"--dry-run".to_string());
    let undo = args.contains(&"--undo".to_string());

    if !target_dir.exists() {
        println!("❌ Target directory does not exist: {}", target_dir.display());
        println!();
        println!("Press Enter to exit...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        std::process::exit(1);
    }

    if !target_dir.is_dir() {
        println!("❌ Target path is not a directory: {}", target_dir.display());
        println!();
        println!("Press Enter to exit...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        std::process::exit(1);
    }

    println!("📁 Folder Organizer - Command Line Version");
    println!("==========================================");
    println!("🎯 Target directory: {}", target_dir.display());
    println!("🔄 Recursive: {}", if recursive { "Yes" } else { "No" });
    println!("👁️  Dry run: {}", if dry_run { "Yes" } else { "No" });
    println!();

    let organizer = FileOrganizer::new(target_dir.clone(), recursive, dry_run);

    if undo {
        println!("🔄 Undoing last organization...");
        match organizer.undo_last_organization() {
            Ok(_) => {
                println!("✅ Undo completed successfully!");
                println!("📁 Files restored to their original locations");
            }
            Err(e) => {
                println!("❌ Error during undo: {}", e);
                println!();
                println!("Press Enter to exit...");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                std::process::exit(1);
            }
        }
    } else {
        println!("🔄 Starting organization...");
        let start_time = std::time::Instant::now();
        
        match organizer.organize() {
            Ok(log) => {
                let duration = start_time.elapsed();
                println!("✅ Organization completed successfully!");
                println!("📊 Summary:");
                println!("  - Total files processed: {}", log.moves.len());
                println!("  - Categories created: {}", 
                    log.moves.iter()
                        .map(|m| &m.category)
                        .collect::<std::collections::HashSet<_>>()
                        .len()
                );
                println!("  - Duration: {:.2?}", duration);
                
                if !dry_run {
                    println!("  - Log saved to: {}/.folder_organizer_log.json", target_dir.display());
                }
                
                // Show some example moves
                if !log.moves.is_empty() {
                    println!();
                    println!("📋 Sample files organized:");
                    for (i, move_entry) in log.moves.iter().take(5).enumerate() {
                        let filename = move_entry.from.file_name().unwrap().to_str().unwrap();
                        println!("  {}. {} -> {}/", i + 1, filename, move_entry.category);
                    }
                    if log.moves.len() > 5 {
                        println!("  ... and {} more files", log.moves.len() - 5);
                    }
                }
            }
            Err(e) => {
                println!("❌ Error during organization: {}", e);
                println!();
                println!("Press Enter to exit...");
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                std::process::exit(1);
            }
        }
    }

    // Always pause at the end so user can see the results
    println!();
    println!("Press Enter to exit...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
} 