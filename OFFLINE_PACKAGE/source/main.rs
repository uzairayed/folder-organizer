mod error;
mod organizer;

use clap::Parser;
use organizer::FileOrganizer;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "folder_organizer",
    about = "A cross-platform command-line tool to automatically organize files into categorized subfolders",
    version,
    long_about = "Organizes files in a directory into categorized subfolders based on file extensions. 
    
Categories include:
- Images (jpg, png, gif, etc.)
- Videos (mp4, avi, mov, etc.)
- Documents (pdf, doc, txt, etc.)
- Audio (mp3, wav, flac, etc.)
- Archives (zip, rar, 7z, etc.)
- Code (py, js, java, etc.)
- Spreadsheets (xls, csv, etc.)
- Presentations (ppt, pptx, etc.)
- Others (uncategorized files)"
)]
struct Args {
    /// Target directory to organize
    #[arg(value_name = "TARGET_DIR")]
    target_dir: PathBuf,

    /// Organize subdirectories recursively
    #[arg(short, long)]
    recursive: bool,

    /// Preview changes without moving files
    #[arg(long)]
    dry_run: bool,

    /// Revert last organization (if log exists)
    #[arg(long)]
    undo: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Validate target directory
    if !args.target_dir.exists() {
        anyhow::bail!("Target directory does not exist: {}", args.target_dir.display());
    }

    if !args.target_dir.is_dir() {
        anyhow::bail!("Target path is not a directory: {}", args.target_dir.display());
    }

    let organizer = FileOrganizer::new(
        args.target_dir.clone(),
        args.recursive,
        args.dry_run,
    );

    if args.undo {
        println!("Undoing last organization in: {}", args.target_dir.display());
        organizer.undo_last_organization()?;
    } else {
        println!(
            "Organizing files in: {} {}",
            args.target_dir.display(),
            if args.recursive { "(recursive)" } else { "" }
        );

        if args.dry_run {
            println!("[DRY RUN MODE] - No files will be moved");
        }

        let start_time = std::time::Instant::now();
        let result = organizer.organize();
        let duration = start_time.elapsed();

        match result {
            Ok(log) => {
                println!(
                    "\n✅ Organization completed in {:.2?}",
                    duration
                );
                println!("📊 Summary:");
                println!("  - Total files processed: {}", log.moves.len());
                println!("  - Categories created: {}", 
                    log.moves.iter()
                        .map(|m| &m.category)
                        .collect::<std::collections::HashSet<_>>()
                        .len()
                );
                
                if !args.dry_run {
                    println!("  - Log saved to: {}/.folder_organizer_log.json", args.target_dir.display());
                }
            }
            Err(e) => {
                eprintln!("❌ Error during organization: {}", e);
                std::process::exit(1);
            }
        }
    }

    Ok(())
} 