use std::{
    fs::{self, File},
    io::Read,
};

use anyhow::{Ok, Result};
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use rand::{distributions::Alphanumeric, Rng};

const PRG: &str = "head";
const EMPTY: &str = "tests/inputs/empty.txt";
const INVALID: &str = "tests/inputs/invalid.txt";
const TEST1: &str = "tests/inputs/test1.txt";
const TEST2: &str = "tests/inputs/test2.txt";
const TEST3: &str = "tests/inputs/test3.txt";
const TEST12: &str = "tests/inputs/test12.txt";

fn gen_rand_filename() -> String {
    loop {
        let fname: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&fname).is_err() {
            return fname;
        }
    }
}

fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let mut file = File::open(expected_file)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let expected = String::from_utf8_lossy(&buffer);

    let output = Command::cargo_bin(PRG)?.args(args).output().expect("fail");
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim_end(), expected);

    Ok(())
}

fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> Result<()> {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(PRG)?
        .write_stdin(input)
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);

    Ok(())
}

#[test]
fn test_not_exist_file() -> Result<()> {
    let bad = gen_rand_filename();
    let expected = format!("{bad}: .* [(]os error 2[)]");
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

#[test]
fn test_empty_file() -> Result<()> {
    Command::cargo_bin(PRG)?
        .arg(EMPTY)
        .assert()
        .success()
        .stderr(predicate::str::is_match("")?);
    Ok(())
}

#[test]
fn test_invalid_file() -> Result<()> {
    // $ touch tests/inputs/invalid.txt && chmod 000 tests/inputs/invalid.txt
    let expected = format!("{INVALID}: .* [(]os error 13[)]");
    Command::cargo_bin(PRG)?
        .arg(INVALID)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

#[test]
fn test_stdin() -> Result<()> {
    run_stdin(TEST1, &["-"], TEST1)
}

#[test]
fn test_text_file1() -> Result<()> {
    run(&[TEST1], TEST1)
}

#[test]
fn test_text_file2() -> Result<()> {
    run(&[TEST2], TEST2)
}

#[test]
fn test_text_file3() -> Result<()> {
    run(&[TEST3], TEST3)
}

#[test]
fn test_text_file12_lines() -> Result<()> {
    run(&["-n", "4", TEST12], "tests/outputs/test12_n4.txt")
}

#[test]
fn test_text_file12_bytes() -> Result<()> {
    run(&["-c", "5", TEST12], "tests/outputs/test12_c5.txt")
}
