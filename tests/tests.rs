#[cfg(test)]
mod tests {
    use std::{fs, io::Cursor, path::PathBuf};
    use md_linker::utils::is_markdown_file;
    use md_linker::files::extract_title;
    use md_linker::render::{generate_table_of_contents, write_file_sections};

    #[test]
    fn test_is_markdown_file() {
        assert!(is_markdown_file(PathBuf::from("file.md").as_path()));
        assert!(!is_markdown_file(PathBuf::from("file.txt").as_path()));
    }

    #[test]
    fn test_extract_title_with_header() {
        let path = PathBuf::from("test_header.md");
        fs::write(&path, "# Hello World\nSome content.").unwrap();
        let title = extract_title(&path);
        fs::remove_file(&path).unwrap();
        assert_eq!(title, Some("Hello World".to_string()));
    }

    #[test]
    fn test_extract_title_with_no_header() {
        let path = PathBuf::from("test_no_header.md");
        fs::write(&path, "No header here.").unwrap();
        let title = extract_title(&path);
        fs::remove_file(&path).unwrap();
        assert_eq!(title, None);
    }

    #[test]
    fn test_generate_table_of_contents_output() {
        let files = vec![
            (PathBuf::from("notes/intro.md"), "Intro".to_string()),
            (PathBuf::from("notes/summary.md"), "Summary".to_string()),
        ];
        let mut buffer = Cursor::new(Vec::new());
        generate_table_of_contents(&mut buffer, &files, PathBuf::from("notes").as_path()).unwrap();
        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert!(output.contains("- [intro.md](#intro)"));
        assert!(output.contains("- [summary.md](#summary)"));
    }

    #[test]
    fn test_write_file_sections_skips_empty() {
        let path = PathBuf::from("empty.md");
        fs::write(&path, "").unwrap();
        let titles = vec![(path.clone(), "Empty File".to_string())];
        let mut buffer = Cursor::new(Vec::new());
        write_file_sections(&mut buffer, &titles, PathBuf::from(".").as_path(), "#").unwrap();
        fs::remove_file(&path).unwrap();
        let output = String::from_utf8(buffer.into_inner()).unwrap();
        assert!(output.contains("# Empty File"));
        assert!(!output.contains("---"));
    }
}
