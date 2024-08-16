use std::fs;

use anyhow::Result;
use assert_cmd::Command;
use pretty_assertions::assert_eq;
use tempfile::NamedTempFile;

const PRG: &str = "uniq";
const EMPTY: Test = Test {
    input: "tests/inputs/empty.txt",
    output: "tests/outputs/empty.txt",
    out_count: "tests/outputs/empty.txt",
};

struct Test {
    input: &'static str,
    output: &'static str,
    out_count: &'static str,
}

fn run(test: &Test) -> Result<()> {
    let expected = fs::read_to_string(test.output)?;
    let output = Command::cargo_bin(PRG)?
        .arg(test.input)
        .output()
        .expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);

    Ok(())
}

fn run_count(test: &Test) -> Result<()> {
    let expected = fs::read_to_string(test.out_count)?;
    let output = Command::cargo_bin(PRG)?
        .args([test.input, "-c"])
        .output()
        .expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);

    Ok(())
}

fn run_stdin(test: &Test) -> Result<()> {
    let input = fs::read_to_string(test.input)?;
    let expected = fs::read_to_string(test.output)?;
    let output = Command::cargo_bin(PRG)?
        .write_stdin(input)
        .output()
        .expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}

fn run_stdin_count(test: &Test) -> Result<()> {
    let input = fs::read_to_string(test.input)?;
    let expected = fs::read_to_string(test.out_count)?;
    let output = Command::cargo_bin(PRG)?
        .arg("--count")
        .write_stdin(input)
        .output()
        .expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);

    Ok(())
}

fn run_outfile(test: &Test) -> Result<()> {
    let expected = fs::read_to_string(test.output)?;
    let outfile = NamedTempFile::new()?;
    let outpath = &outfile.path().to_str().unwrap();
    Command::cargo_bin(PRG)?
        .args([test.input, outpath])
        .assert()
        .success()
        .stdout("");
    let contents = fs::read_to_string(&outpath)?;
    assert_eq!(&expected, &contents);

    Ok(())
}

fn run_outfile_count(test: &Test) -> Result<()> {
    let outfile = NamedTempFile::new()?;
    let outpath = &outfile.path().to_str().unwrap();

    Command::cargo_bin(PRG)?
        .args([test.input, outpath, "--count"])
        .assert()
        .success()
        .stdout("");

    let expected = fs::read_to_string(test.out_count)?;
    let contents = fs::read_to_string(outpath)?;
    assert_eq!(&expected, &contents);

    Ok(())
}

fn run_stdin_outfile_count(test: &Test) -> Result<()> {
    let input = fs::read_to_string(test.input)?;
    let outfile = NamedTempFile::new()?;
    let outpath = &outfile.path().to_str().unwrap();

    Command::cargo_bin(PRG)?
        .args(["-", outpath, "-c"])
        .write_stdin(input)
        .assert()
        .stdout("");

    let expected = fs::read_to_string(test.out_count)?;
    let contents = fs::read_to_string(outpath)?;
    assert_eq!(&expected, &contents);

    Ok(())
}

#[test]
fn test_empty() -> Result<()> {
    run(&EMPTY)
}

#[test]
fn test_empty_count() -> Result<()> {
    run_count(&EMPTY)
}

#[test]
fn test_empty_stdin() -> Result<()> {
    run_stdin(&EMPTY)
}

#[test]
fn test_empty_stdin_count() -> Result<()> {
    run_stdin_count(&EMPTY)
}

#[test]
fn test_empty_outfile() -> Result<()> {
    run_outfile(&EMPTY)
}

#[test]
fn empty_outfile_count() -> Result<()> {
    run_outfile_count(&EMPTY)
}

#[test]
fn empty_stdin_outfile_count() -> Result<()> {
    run_stdin_outfile_count(&EMPTY)
}
