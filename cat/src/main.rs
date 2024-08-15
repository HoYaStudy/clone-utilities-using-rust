use std::fs::File;
use std::io::{self, BufRead, BufReader};

use anyhow::Result;

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
    for fname in args.files {
        match open(&fname) {
            Err(err) => eprintln!("Failed to open {fname}: {err}"),
            Ok(file) => {
                let mut prev_num = 0;
                for (line_num, line) in file.lines().enumerate() {
                    let line = line?;

                    if args.number_lines {
                        println!("{:>6}\t{line}", line_num + 1);
                    } else if args.number_nonblank_lines {
                        if line.is_empty() {
                            println!();
                        } else {
                            prev_num += 1;
                            println!("{prev_num:>6}\t{line}");
                        }
                    } else {
                        println!("{line}");
                    }
                }
            }
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
