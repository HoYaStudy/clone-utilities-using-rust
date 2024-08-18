use std::fs::File;
use std::io::{self, BufRead, BufReader};

use anyhow::{anyhow, bail, Result};

use crate::Column::*;

// mod builder;
mod derive;

enum Column<'a> {
    Col1(&'a str),
    Col2(&'a str),
    Col3(&'a str),
}

fn main() {
    // if let Err(e) = run(builder::builder_pattern()) {
    if let Err(e) = run(derive::derive_pattern()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

// fn run(args: builder::Args) -> Result<()> {
fn run(args: derive::Args) -> Result<()> {
    let file1 = &args.file1;
    let file2 = &args.file2;

    if file1 == "-" && file2 == "-" {
        bail!(r#"Both input files cannot be STDIN ("-")"#);
    }

    let case = |line: String| {
        if args.insensitive {
            line.to_lowercase()
        } else {
            line
        }
    };

    let mut lines1 = open(file1)?.lines().map_while(Result::ok).map(case);
    let mut lines2 = open(file2)?.lines().map_while(Result::ok).map(case);

    let print = |col: Column| {
        let mut columns = vec![];
        match col {
            Col1(val) => {
                if args.show_col1 {
                    columns.push(val);
                }
            }
            Col2(val) => {
                if args.show_col2 {
                    if args.show_col1 {
                        columns.push("");
                    }
                    columns.push(val);
                }
            }
            Col3(val) => {
                if args.show_col3 {
                    if args.show_col1 {
                        columns.push("");
                    }
                    if args.show_col2 {
                        columns.push("");
                    }
                    columns.push(val);
                }
            }
        };

        if !columns.is_empty() {
            println!("{}", columns.join(&args.delimiter));
        }
    };

    let mut line1 = lines1.next();
    let mut line2 = lines2.next();

    while line1.is_some() || line2.is_some() {
        match (&line1, &line2) {
            (Some(val1), Some(val2)) => match val1.cmp(val2) {
                equal => {
                    print(Col3(val1));
                    line1 = lines1.next();
                    line2 = lines2.next();
                }
                less => {
                    print(Col1(val1));
                    line1 = lines1.next();
                }
                greater => {
                    print(Col2(val2));
                    line2 = lines2.next();
                }
            },
            (Some(val1), None) => {
                print(Col1(val1));
                line1 = lines1.next();
            }
            (None, Some(val2)) => {
                print(Col2(val2));
                line2 = lines2.next();
            }
            _ => (),
        }
    }

    Ok(())
}

fn open(fname: &str) -> Result<Box<dyn BufRead>> {
    match fname {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(fname).map_err(|e| anyhow!("{fname}: {e}"))?,
        ))),
    }
}
