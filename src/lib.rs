//! grss_clone is a command line search tool that finds occurrences of a pattern in a file.
//!
//! # Features
//!
//! - Search for text patterns in files
//! - Case-sensitive search
//! - Line-by-line output of matches
//!
//! # Performance
//!
//! The tool reads files line by line to handle large files efficiently without
//! loading the entire file into memory.

pub fn find_matches(
    content: &str,
    pattern: &str,
    mut writer: impl std::io::Write,
) -> std::io::Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}
