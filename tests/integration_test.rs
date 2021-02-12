use assert_cmd::{assert::OutputAssertExt, prelude::CommandCargoExt};
use predicates::prelude::*;
use std::process::Command;

#[test]
fn include_none_didit() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("kenall-rs")?;
    cmd.arg("100-0012");
    cmd.assert().success();

    Ok(())
}

#[test]
fn more_than_seven_digits() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("kenall-rs")?;
    cmd.arg("1000012909090909090");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Please enter the postcode with 7digit like following: `kenall-rs 1000000` or `kenall-rs 100-0000`"));

    Ok(())
}

#[test]
fn less_than_seven_digits() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("kenall-rs")?;
    cmd.arg("123");
    cmd.assert()
    .failure()
    .stderr(predicate::str::contains("Please enter the postcode with 7digit like following: `kenall-rs 1000000` or `kenall-rs 100-0000`"));

    Ok(())
}
