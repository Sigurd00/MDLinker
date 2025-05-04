use std::{io::Write, path::{Path, PathBuf}};

pub fn generate_table_of_contents<W: Write>(
    writer: &mut W,
    file_titles: &[(PathBuf, String)],
    base_dir: &Path,
) -> std::io::Result<()> {
    writeln!(writer, "# Table of Contents\n")?;
    for (path, title) in file_titles {
        let anchor = title.replace(' ', "-").to_lowercase();
        let relative = path.strip_prefix(base_dir).unwrap_or(path);
        writeln!(writer, "- [{}](#{})", relative.display(), anchor)?;
    }
    Ok(())
}

pub fn write_file_sections<W: Write>(
    writer: &mut W,
    file_titles: &[(PathBuf, String)],
    base_dir: &Path,
    header_level: &str,
) -> std::io::Result<()> {
    use std::fs;

    for (path, title) in file_titles {
        let relative = path.strip_prefix(base_dir).unwrap_or(path);

        writeln!(writer, "{} {}\n", header_level, title)?;

        match fs::read_to_string(path) {
            Ok(contents) => {
                if contents.trim().is_empty() {
                    eprintln!("Skipped empty file: {}", path.display());
                    continue;
                }
                writer.write_all(contents.as_bytes())?;
                writeln!(writer, "\n---\n")?;
                println!("Added: {}", relative.display());
            }
            Err(e) => {
                eprintln!("Could not read {} ({})", path.display(), e);
            }
        }
    }
    Ok(())
}