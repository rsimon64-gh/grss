use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;
use std::fs;
use tempfile::NamedTempFile;

#[test]
fn find_content_in_file() -> Result<(), Box<dyn Error>> {
    let file = NamedTempFile::new()?;
    fs::write(&file, "A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grep_clone")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nAnother test"));

    Ok(())
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("grep_clone")?;
    cmd.arg("test").arg("non_existent_file.txt");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}
