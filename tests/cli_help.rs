use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

#[test]
fn help_message_on_no_args() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("grrs_clone")?;

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));

    Ok(())
}

#[test]
fn help_message_on_help_flag() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("grrs_clone")?;

    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("The pattern to look for"))
        .stdout(predicate::str::contains("The path to the file to read"));

    Ok(())
}

#[test]
fn help_message_on_version_flag() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("grrs_clone")?;

    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("grrs_clone"));

    Ok(())
}
