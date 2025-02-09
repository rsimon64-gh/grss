//! grss_clone is a command line search tool that finds occurrences of a pattern in a file.
//!
//! # Features
//!
//! - Search for text patterns in files
//! - Case-sensitive search
//! - Line-by-line output of matches
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```no_run
//! use grss_clone;
//! use std::io;
//!
//! let pattern = "pattern";
//! let content = "some text\nwith pattern\nmore text";
//! grss_clone::find_matches(content, pattern, io::stdout()).unwrap();
//! ```
//!
//! # Error Handling
//!
//! The library uses `anyhow` for error handling, providing detailed error messages
//! when operations fail, such as:
//! - File not found
//! - Permission denied
//! - Invalid UTF-8 in files
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
