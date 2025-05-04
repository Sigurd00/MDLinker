// src/main.rs
mod cli;
mod files;
mod render;
mod utils;

use clap::Parser;
use std::{fs, fs::File, io::{BufWriter, Write}, path::Path, time::Instant};
use cli::Cli;
use files::{collect_md_files, extract_title};
use render::{generate_table_of_contents, write_file_sections};

fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let start_time = Instant::now();

    let notes_dir = Path::new(&args.input);
    let mut md_files = Vec::new();
    collect_md_files(notes_dir, &mut md_files)?;
    md_files.sort();

    let temp_output_path = format!("{}.temp", &args.output);
    let file = File::create(&temp_output_path)?;
    let mut output = BufWriter::new(file);

    let mut file_titles = md_files.iter()
        .map(|path| {
            let title = extract_title(path).unwrap_or_else(|| {
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Unnamed")
                    .to_string()
            });
            (path.clone(), title)
        })
        .collect::<Vec<_>>();

    generate_table_of_contents(&mut output, &file_titles, notes_dir)?;
    writeln!(output, "\n---\n")?;
    write_file_sections(&mut output, &file_titles, notes_dir, &args.header_level)?;

    fs::rename(temp_output_path, &args.output)?;
    println!(
        "Done. Wrote to {} in {:.2?}. Processed {} Markdown file(s).",
        args.output,
        start_time.elapsed(),
        file_titles.len()
    );

    Ok(())
}
