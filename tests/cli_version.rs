use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

#[test]
fn version_flag_works() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("grrs_clone")?;

    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("grrs_clone 0.3.0"));

    Ok(())
}
