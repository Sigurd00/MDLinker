use clap::Parser;

#[derive(Parser)]
#[command(name = "md-compiler", about = "Concatenate Markdown files recursively")]
pub struct Cli {
    /// Input directory
    #[arg(short, long, default_value = "./notes")]
    pub input: String,

    /// Output file path
    #[arg(short, long, default_value = "output.md")]
    pub output: String,

    /// Markdown header level (e.g. # or ##)
    #[arg(short = 'l', long, default_value = "#")]
    pub header_level: String,
}