use std::{fs, path::{PathBuf}};

#[derive(Debug)]
pub struct FileInfo {
    pub path: PathBuf,
    pub extension: Option<String>,
}

impl FileInfo {
    fn from_path(path: PathBuf) -> Self {
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.to_lowercase());

        Self {
            path,
            extension
        }
    }
}

pub fn scan_dir(dir: &str) -> Vec<FileInfo> {
    let entries = fs::read_dir(dir).unwrap();
    let mut files: Vec<FileInfo> = Vec::new();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let info = FileInfo::from_path(path);
            files.push(info);
        }
    }

    files
}