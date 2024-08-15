use std::fs;

use anyhow::{Ok, Result};
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use rand::{distributions::Alphanumeric, Rng};

const PRG: &str = "cat";
const EMPTY: &str = "tests/inputs/empty.txt";
const INVALID: &str = "tests/inputs/invalid.txt";
const TEST1: &str = "tests/inputs/test1.txt";
const TEST2: &str = "tests/inputs/test2.txt";
const TEST3: &str = "tests/inputs/test3.txt";

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
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(PRG)?.args(args).output().unwrap();
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout.trim_end(), expected);

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
    assert_eq!(stdout.trim_end(), expected);

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
fn test_text_file3_linenum() -> Result<()> {
    run(&["-n", TEST3], "tests/outputs/test3_n.txt")
}

#[test]
fn test_text_file3_nonblank_linenum() -> Result<()> {
    run(&["-b", TEST3], "tests/outputs/test3_b.txt")
}
