use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;

const PRG: &str = "cut";
const CSV: &str = "tests/inputs/test.csv";

fn dies(args: &[&str], expected: &str) -> Result<()> {
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

#[test]
fn test_bad_delimiter() -> Result<()> {
    dies(
        &[CSV, "-f", "1", "-d", ",,"],
        r#"--delim ",," must be a single byte"#,
    )
}
