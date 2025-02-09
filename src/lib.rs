//! grss_clone is a command line search tool that finds occurrences of a pattern in a file.
//!
//! # Features
//!
//! - Search for text patterns in files
//! - Search using regular expressions
//! - Line-by-line output of matches
//!
//! # Performance
//!
//! The tool reads files line by line to handle large files efficiently without
//! loading the entire file into memory.

use regex::Regex;
use std::io::Write;

pub fn find_matches(content: &str, pattern: &str, mut writer: impl Write) -> std::io::Result<()> {
    let re = Regex::new(pattern)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e))?;
    for line in content.lines() {
        if re.is_match(line.trim()) {
            writeln!(writer, "{}", line.trim())?;
        }
    }
    Ok(())
}
