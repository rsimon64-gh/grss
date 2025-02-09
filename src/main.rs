// Add this at the top of the file
#![doc(html_root_url = "https://docs.rs/grep_clone")]
use anyhow::Result;
use clap::Parser;

/// A simple implementation of the grep command line tool
///
/// This program searches for patterns in text files and prints matching lines.
/// It is similar to the Unix grep utility but with a simplified feature set.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The pattern to look for in the file
    #[arg(help = "The pattern to look for")]
    pattern: String,

    /// The path to the file to read
    #[arg(help = "The path to the file to read")]
    path: std::path::PathBuf,
}

/// Finds matches of a pattern in a file and prints them to stdout
///
/// # Arguments
///
/// * `pattern` - The text pattern to search for
/// * `path` - The path to the file to search in
///
/// # Returns
///
/// * `Result<()>` - Ok if the search completed successfully, Err if an error occurred
///
/// # Examples
///
/// ```no_run
/// use grep_clone::find_matches;
///
/// let pattern = "TODO";
/// let path = "src/main.rs";
/// find_matches(pattern, path).unwrap();
/// ```
///
/// # Errors
///
/// This function will return an error if:
/// * The file does not exist
/// * The program doesn't have permission to read the file
/// * The file contains invalid UTF-8
pub fn find_matches(pattern: &str, path: &std::path::PathBuf) -> Result<()> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}

/// Main entry point for the grep_clone application
///
/// Parses command line arguments and searches for the specified pattern
/// in the given file.
fn main() -> Result<()> {
    let args = Args::parse();
    find_matches(&args.pattern, &args.path)?;
    Ok(())
}
