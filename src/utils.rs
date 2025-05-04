use std::path::Path;

pub fn is_markdown_file(path: &Path) -> bool {
    path.extension()
        .and_then(|s| s.to_str())
        .map(|ext| ext == "md")
        .unwrap_or(false)
}