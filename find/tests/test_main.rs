use std::{borrow::Cow, fs};

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use rand::{distributions::Alphanumeric, Rng};

const PRG: &str = "find";

fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

#[cfg(windows)]
fn format_file_name(expected_file: &str) -> Cow<str> {
    format!("{}.windows", expected_file).into()
}

#[cfg(not(windows))]
fn format_file_name(expected_file: &str) -> Cow<str> {
    expected_file.into()
}

fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let file = format_file_name(expected_file);
    let contents = fs::read_to_string(file.as_ref())?;
    let mut expected: Vec<&str> = contents.split('\n').filter(|s| !s.is_empty()).collect();
    expected.sort();

    let cmd = Command::cargo_bin(PRG)?.args(args).assert().success();
    let out = cmd.get_output();
    let stdout = String::from_utf8(out.stdout.clone())?;
    let mut lines: Vec<&str> = stdout.split('\n').filter(|s| !s.is_empty()).collect();
    lines.sort();

    assert_eq!(lines, expected);

    Ok(())
}

#[test]
fn test_bad_dir() -> Result<()> {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error [23][)]", &bad);
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

#[test]
fn test_bad_name() -> Result<()> {
    Command::cargo_bin(PRG)?
        .args(["--name", "*.csv"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("error: invalid value '*.csv'"));
    Ok(())
}

#[test]
fn test_bad_type() -> Result<()> {
    let expected = "error: invalid value 'x' for '--type [<TYPE>...]'";
    Command::cargo_bin(PRG)?
        .args(["--type", "x"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

#[test]
fn test_path_src() -> Result<()> {
    run(&["src"], "tests/expected/path.txt")
}
