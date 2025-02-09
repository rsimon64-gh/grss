use assert_cmd::Command;
use predicates::prelude::*;
use std::error::Error;

#[test]
fn version_flag_works() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("grss_clone")?;

    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("grss_clone 0.4.2"));

    Ok(())
}
