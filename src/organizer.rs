use crate::error::Result; use std::path::PathBuf; pub struct FileOrganizer { pub target_dir: PathBuf, } impl FileOrganizer { pub fn new(target_dir: PathBuf) -> Self { Self { target_dir } } }
