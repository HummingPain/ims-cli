use std::io::Write;
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
pub struct Cli {
    /// The pattern to look for
    pub pattern: String,
    /// The path to the file to read
    pub path: std::path::PathBuf,
}
pub fn find_matches(args: &Cli, content: String, mut writer: impl Write) {
    for line in content.lines() {
        if line.contains(&args.pattern) {
            writeln!(writer, "{}", line).expect("TODO: panic message");
        }
    }
}
