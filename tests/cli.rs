use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn test_find_matches() {
    let content = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me.";

    let pattern = "safe|productive";
    let mut result = Vec::new();
    grss_clone::find_matches(content, pattern, &mut result).unwrap();
    let result = String::from_utf8(result).unwrap();
    assert_eq!(result, "safe, fast, productive.\n");
}

#[test]
fn test_find_matches_phrase() {
    let content = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me.
        This phrase does not exist.";

    let pattern = "does .* exist";
    let mut result = Vec::new();
    grss_clone::find_matches(content, pattern, &mut result).unwrap();
    let result = String::from_utf8(result).unwrap();
    assert_eq!(result, "This phrase does not exist.\n");
}

#[test]
fn test_find_matches_no_match() {
    let content = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Trust me.";

    let pattern = "notfound";
    let mut result = Vec::new();
    grss_clone::find_matches(content, pattern, &mut result).unwrap();
    let result = String::from_utf8(result).unwrap();
    assert_eq!(result, "");
}

#[test]
fn test_find_matches_invalid_regex() {
    let content = "Rust";
    let pattern = "("; // Invalid regex
    let mut result = Vec::new();
    let error = grss_clone::find_matches(content, pattern, &mut result).unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn Error>> {
    let file = NamedTempFile::new()?;
    fs::write(&file, "A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grss_clone")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));

    Ok(())
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("grss_clone")?;
    cmd.arg("test").arg("non_existent_file.txt");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn find_no_matches() -> Result<(), Box<dyn Error>> {
    let file = NamedTempFile::new()?;
    fs::write(&file, "Some content\nMore content\nEven more content")?;

    let mut cmd = Command::cargo_bin("grss_clone")?;
    cmd.arg("test").arg(file.path());
    cmd.assert().success().stdout(predicate::str::is_empty());

    Ok(())
}

#[test]
fn find_matches_with_special_characters() -> Result<(), Box<dyn Error>> {
    let file = NamedTempFile::new()?;
    fs::write(&file, "Special characters: !@#$%^&*()\nAnother line")?;

    let mut cmd = Command::cargo_bin("grss_clone")?;
    cmd.arg("!@#$%^&*()").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Special characters: !@#$%^&*()"));

    Ok(())
}

#[test]
fn find_matches_with_utf8_characters() -> Result<(), Box<dyn Error>> {
    let file = NamedTempFile::new()?;
    fs::write(&file, "UTF-8 characters: 你好，世界\nAnother line")?;

    let mut cmd = Command::cargo_bin("grss_clone")?;
    cmd.arg("你好").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("UTF-8 characters: 你好，世界"));

    Ok(())
}

#[test]
fn find_matches_with_multiple_patterns() -> Result<(), Box<dyn Error>> {
    let file = NamedTempFile::new()?;
    fs::write(&file, "First pattern\nSecond pattern\nAnother line")?;

    let mut cmd = Command::cargo_bin("grss_clone")?;
    cmd.arg("pattern").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("First pattern\nSecond pattern"));

    Ok(())
}

#[test]
fn find_matches_in_large_file() -> Result<(), Box<dyn Error>> {
    let file = NamedTempFile::new()?;
    let content = "test\n".repeat(10000);
    fs::write(&file, &content)?;

    let mut cmd = Command::cargo_bin("grss_clone")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("test").count(10000));

    Ok(())
}
