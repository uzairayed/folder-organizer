use crate::error::{OrganizerError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveLog {
    pub timestamp: String,
    pub moves: Vec<MoveEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveEntry {
    pub from: PathBuf,
    pub to: PathBuf,
    pub category: String,
}

pub struct FileOrganizer {
    categories: HashMap<String, Vec<String>>,
    target_dir: PathBuf,
    recursive: bool,
    dry_run: bool,
}

impl FileOrganizer {
    pub fn new(target_dir: PathBuf, recursive: bool, dry_run: bool) -> Self {
        let categories = Self::get_default_categories();
        Self {
            categories,
            target_dir,
            recursive,
            dry_run,
        }
    }

    fn get_default_categories() -> HashMap<String, Vec<String>> {
        HashMap::from([
            ("Images".to_string(), vec!["jpg".to_string(), "jpeg".to_string(), "png".to_string(), "gif".to_string(), "bmp".to_string(), "tiff".to_string(), "webp".to_string(), "svg".to_string()]),
            ("Videos".to_string(), vec!["mp4".to_string(), "avi".to_string(), "mov".to_string(), "wmv".to_string(), "flv".to_string(), "mkv".to_string(), "webm".to_string(), "m4v".to_string()]),
            ("Documents".to_string(), vec!["pdf".to_string(), "doc".to_string(), "docx".to_string(), "txt".to_string(), "rtf".to_string(), "odt".to_string(), "pages".to_string()]),
            ("Audio".to_string(), vec!["mp3".to_string(), "wav".to_string(), "flac".to_string(), "aac".to_string(), "ogg".to_string(), "wma".to_string(), "m4a".to_string()]),
            ("Archives".to_string(), vec!["zip".to_string(), "rar".to_string(), "7z".to_string(), "tar".to_string(), "gz".to_string(), "bz2".to_string(), "xz".to_string()]),
            ("Code".to_string(), vec!["py".to_string(), "js".to_string(), "ts".to_string(), "java".to_string(), "cpp".to_string(), "c".to_string(), "h".to_string(), "rs".to_string(), "go".to_string(), "php".to_string(), "html".to_string(), "css".to_string(), "xml".to_string(), "json".to_string(), "yaml".to_string(), "toml".to_string(), "md".to_string()]),
            ("Spreadsheets".to_string(), vec!["xls".to_string(), "xlsx".to_string(), "csv".to_string(), "ods".to_string(), "numbers".to_string()]),
            ("Presentations".to_string(), vec!["ppt".to_string(), "pptx".to_string(), "key".to_string(), "odp".to_string()]),
        ])
    }

    pub fn organize(&self) -> Result<MoveLog> {
        if !self.target_dir.exists() {
            return Err(OrganizerError::InvalidTargetDir {
                path: self.target_dir.clone(),
            });
        }

        if !self.target_dir.is_dir() {
            return Err(OrganizerError::InvalidTargetDir {
                path: self.target_dir.clone(),
            });
        }

        let mut move_log = MoveLog {
            timestamp: chrono::Utc::now().to_rfc3339(),
            moves: Vec::new(),
        };

        let walker = if self.recursive {
            WalkDir::new(&self.target_dir)
                .min_depth(1)
                .max_depth(usize::MAX)
        } else {
            WalkDir::new(&self.target_dir).min_depth(1).max_depth(1)
        };

        for entry in walker {
            let entry = entry.map_err(|e| OrganizerError::ReadDirError {
                path: self.target_dir.clone(),
                source: e.into_io_error().unwrap_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::Other, "WalkDir error")
                }),
            })?;

            let path = entry.path();
            if path.is_file() {
                self.process_file(path, &mut move_log)?;
            }
        }

        if !self.dry_run {
            self.save_log(&move_log)?;
        }

        Ok(move_log)
    }

    fn process_file(&self, file_path: &Path, move_log: &mut MoveLog) -> Result<()> {
        let extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase())
            .ok_or_else(|| OrganizerError::NoExtensionError {
                path: file_path.to_path_buf(),
            })?;

        let category = self.get_category_for_extension(&extension);
        let target_dir = self.target_dir.join(category);

        if !self.dry_run {
            std::fs::create_dir_all(&target_dir).map_err(|e| OrganizerError::CreateDirError {
                path: target_dir.clone(),
                source: e,
            })?;
        }

        let filename = file_path.file_name().unwrap();
        let mut target_path = target_dir.join(filename);

        // Handle duplicate filenames
        let mut counter = 1;
        while target_path.exists() && !self.dry_run {
            let stem = file_path.file_stem().unwrap();
            let extension_str = file_path.extension().unwrap();
            let new_filename = format!(
                "{}_{}.{}",
                stem.to_str().unwrap(),
                counter,
                extension_str.to_str().unwrap()
            );
            target_path = target_dir.join(new_filename);
            counter += 1;
        }

        if !self.dry_run {
            std::fs::rename(file_path, &target_path).map_err(|e| {
                OrganizerError::MoveFileError {
                    from: file_path.to_path_buf(),
                    to: target_path.clone(),
                    source: e,
                }
            })?;
        }

        let target_filename = target_path.file_name().unwrap().to_str().unwrap().to_string();
        
        move_log.moves.push(MoveEntry {
            from: file_path.to_path_buf(),
            to: target_path.clone(),
            category: category.to_string(),
        });

        println!(
            "{} {} -> {}/{}",
            if self.dry_run { "[DRY RUN]" } else { "[MOVE]" },
            file_path.file_name().unwrap().to_str().unwrap(),
            category,
            target_filename
        );

        Ok(())
    }

    fn get_category_for_extension(&self, extension: &str) -> &str {
        for (category, extensions) in &self.categories {
            if extensions.contains(&extension.to_string()) {
                return category;
            }
        }
        "Others"
    }

    fn save_log(&self, log: &MoveLog) -> Result<()> {
        let log_path = self.target_dir.join(".folder_organizer_log.json");
        let log_content = serde_json::to_string_pretty(log)
            .map_err(|e| OrganizerError::SerializationError { source: e })?;

        std::fs::write(&log_path, log_content)
            .map_err(|e| OrganizerError::LogWriteError { source: e })?;

        Ok(())
    }

    pub fn undo_last_organization(&self) -> Result<()> {
        let log_path = self.target_dir.join(".folder_organizer_log.json");
        
        if !log_path.exists() {
            println!("No organization log found. Nothing to undo.");
            return Ok(());
        }

        let log_content = std::fs::read_to_string(&log_path)
            .map_err(|e| OrganizerError::LogWriteError { source: e })?;

        let log: MoveLog = serde_json::from_str(&log_content)
            .map_err(|e| OrganizerError::SerializationError { source: e })?;

        println!("Undoing {} file moves...", log.moves.len());

        for move_entry in log.moves.iter().rev() {
            if move_entry.to.exists() {
                std::fs::rename(&move_entry.to, &move_entry.from)
                    .map_err(|e| OrganizerError::MoveFileError {
                        from: move_entry.to.clone(),
                        to: move_entry.from.clone(),
                        source: e,
                    })?;
                println!("Undid: {} -> {}", move_entry.to.display(), move_entry.from.display());
            }
        }

        // Remove empty directories
        for category in self.categories.keys() {
            let category_path = self.target_dir.join(category);
            if category_path.exists() && category_path.read_dir().unwrap().next().is_none() {
                std::fs::remove_dir(category_path).ok();
            }
        }

        // Remove the log file
        std::fs::remove_file(log_path).ok();

        println!("Undo completed successfully!");
        Ok(())
    }
} 