use anyhow::{Ok, Result};
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_without_args() -> Result<()> {
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn test_with_arg_text() -> Result<()> {
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.arg("hello").assert().success();
    Ok(())
}

#[test]
fn test_with_arg_text_newline() -> Result<()> {
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.arg("Hello HoYa")
        .assert()
        .success()
        .stdout("Hello HoYa\n");

    let mut cmd = Command::cargo_bin("echo")?;
    cmd.args(vec!["Hello", "HoYa"])
        .assert()
        .success()
        .stdout("Hello HoYa\n");
    Ok(())
}

#[test]
fn test_with_arg_text_no_newline() -> Result<()> {
    let mut cmd = Command::cargo_bin("echo")?;
    cmd.arg("-n")
        .arg("Hello HoYa")
        .assert()
        .success()
        .stdout("Hello HoYa");

    let mut cmd = Command::cargo_bin("echo")?;
    cmd.args(vec!["-n", "Hello", "HoYa"])
        .assert()
        .success()
        .stdout("Hello HoYa");
    Ok(())
}
