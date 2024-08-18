use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::mem;

use anyhow::{anyhow, Result};
use regex::{Regex, RegexBuilder};
use walkdir::WalkDir;

// mod builder;
mod derive;

fn main() {
    // if let Err(e) = run(builder::builder_pattern()) {
    if let Err(e) = run(derive::derive_pattern()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

// fn run(args: builder::Args) -> Result<()> {
fn run(args: derive::Args) -> Result<()> {
    let pattern = RegexBuilder::new(&args.pattern)
        .case_insensitive(args.insensitive)
        .build()
        .map_err(|_| anyhow!(r#"Invalid pattern "{}""#, args.pattern))?;

    let entries = find_files(&args.files, args.recursive);
    let num_files = entries.len();
    let print = |fname: &str, val: &str| {
        if num_files > 1 {
            print!("{fname}:{val}");
        } else {
            print!("{val}");
        }
    };

    for entry in entries {
        match entry {
            Err(e) => eprintln!("{e}"),
            Ok(filename) => match open(&filename) {
                Err(e) => eprintln!("{filename}: {e}"),
                Ok(file) => match find_lines(file, &pattern, args.invert) {
                    Err(e) => eprintln!("{e}"),
                    Ok(matches) => {
                        if args.count {
                            print(&filename, &format!("{}\n", matches.len()));
                        } else {
                            for line in &matches {
                                print(&filename, line);
                            }
                        }
                    }
                },
            },
        }
    }

    Ok(())
}

fn open(fname: &str) -> Result<Box<dyn BufRead>> {
    match fname {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(fname)?))),
    }
}

fn find_files(paths: &[String], recursive: bool) -> Vec<Result<String>> {
    let mut results = vec![];

    for path in paths {
        match path.as_str() {
            "-" => results.push(Ok(path.to_string())),
            _ => match fs::metadata(path) {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        if recursive {
                            for entry in WalkDir::new(path)
                                .into_iter()
                                .flatten()
                                .filter(|e| e.file_type().is_file())
                            {
                                results.push(Ok(entry.path().display().to_string()));
                            }
                        } else {
                            results.push(Err(anyhow!("{path} is a directory")));
                        }
                    } else if metadata.is_file() {
                        results.push(Ok(path.to_string()));
                    }
                }
                Err(e) => results.push(Err(anyhow!("{path}: {e}"))),
            },
        }
    }

    results
}

fn find_lines<T: BufRead>(mut file: T, pattern: &Regex, invert: bool) -> Result<Vec<String>> {
    let mut matches = vec![];
    let mut line = String::new();

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if pattern.is_match(&line) ^ invert {
            matches.push(mem::take(&mut line));
        }
        line.clear();
    }

    Ok(matches)
}

#[cfg(test)]
mod tests {
    use super::find_lines;
    use pretty_assertions::assert_eq;
    use regex::{Regex, RegexBuilder};
    use std::io::Cursor;

    #[test]
    fn test_find_lines() {
        let text = b"Lorem\nIpsum\r\nDOLOR";

        // The pattern _or_ should match the one line, "Lorem"
        let re1 = Regex::new("or").unwrap();
        let matches = find_lines(Cursor::new(&text), &re1, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);

        // When inverted, the function should match the other two lines
        let matches = find_lines(Cursor::new(&text), &re1, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        // This regex will be case-insensitive
        let re2 = RegexBuilder::new("or")
            .case_insensitive(true)
            .build()
            .unwrap();

        // The two lines "Lorem" and "DOLOR" should match
        let matches = find_lines(Cursor::new(&text), &re2, false);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 2);

        // When inverted, the one remaining line should match
        let matches = find_lines(Cursor::new(&text), &re2, true);
        assert!(matches.is_ok());
        assert_eq!(matches.unwrap().len(), 1);
    }
}
