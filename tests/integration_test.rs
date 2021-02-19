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
fn normal_eight_digit() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("kenall-rs")?;
    cmd.arg("1000001");
    cmd.assert().stdout(predicate::str::contains("千代田区"));

    Ok(())
}

#[test]
fn kyoto_street_address() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("kenall-rs")?;
    cmd.arg("6048012");
    cmd.assert()
        .stdout(predicate::str::contains("先斗町通三条下る"));

    Ok(())
}

#[test]
fn no_result_found() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("kenall-rs")?;
    cmd.arg("9999999");

    cmd.assert().failure().stderr(predicate::str::contains(
        "Sorry, there was no address associated with the post code",
    ));

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
