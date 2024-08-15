#[cfg(test)]
mod tests {
    use std::{
        fs::{self},
        io::Cursor,
    };

    use anyhow::Result;

    use assert_cmd::Command;
    use wc::{count, format_field, FileInfo};

    const PRG: &str = "wc";
    const EMPTY: &str = "tests/inputs/empty.txt";
    const TEST1: &str = "tests/inputs/test1.txt";
    const TEST2: &str = "tests/inputs/test2.txt";

    #[test]
    fn test_count() {
        let text = "I don't want the world.\nI just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());

        println!("{info:#?}");

        let expected = FileInfo {
            num_lines: 2,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }

    #[test]
    fn test_empty_file() -> Result<()> {
        run(&[EMPTY, TEST1, TEST2], "tests/outputs/all.txt")
    }

    #[test]
    fn test_text_file1() -> Result<()> {
        run(&[TEST1], "tests/outputs/test1.txt")
    }

    #[test]
    fn test_text_file2() -> Result<()> {
        run(&[TEST2], "tests/outputs/test2.txt")
    }

    fn run(args: &[&str], expected_file: &str) -> Result<()> {
        let expected = fs::read_to_string(expected_file)?;
        let output = Command::cargo_bin(PRG)?.args(args).output().expect("fail");
        assert!(output.status.success());

        let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
        assert_eq!(stdout.trim_end(), expected);

        Ok(())
    }
}
