use std::{fs, path::{Path, PathBuf}};

use crate::utils::is_markdown_file;

pub fn collect_md_files(dir: &Path, md_files: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_md_files(&path, md_files)?;
        } else if is_markdown_file(&path) {
            md_files.push(path);
        }
    }
    Ok(())
}

pub fn extract_title(path: &Path) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    for line in content.lines() {
        let trimmed = line.trim_start();
        if trimmed.starts_with("# ") || trimmed.starts_with("## ") {
            return Some(trimmed.trim_start_matches('#').trim().to_string());
        }
    }
    None
}