use std::fs::File;
use std::io::{self, BufRead, BufReader};

use anyhow::Result;

use wc::{count, format_field};

// mod builder;
mod derive;

fn main() {
    // if let Err(e) = run(builder::builder_pattern()) {
    if let Err(e) = run(derive::derive_pattern()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

// fn run(mut args: builder::Args) -> Result<()> {
fn run(mut args: derive::Args) -> Result<()> {
    if [args.lines, args.words, args.bytes, args.chars]
        .iter()
        .all(|v| v == &false)
    {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    for fname in &args.files {
        match open(fname) {
            Err(err) => eprintln!("{fname}: {err}"),
            Ok(file) => {
                let info = count(file)?;
                println!(
                    "{}{}{}{}{}",
                    format_field(info.num_lines, args.lines),
                    format_field(info.num_words, args.words),
                    format_field(info.num_bytes, args.bytes),
                    format_field(info.num_chars, args.chars),
                    if fname == "-" {
                        "".to_string()
                    } else {
                        format!(" {fname}")
                    }
                );

                total_lines += info.num_lines;
                total_words += info.num_words;
                total_bytes += info.num_bytes;
                total_chars += info.num_chars;
            }
        }
    }

    if args.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, args.lines),
            format_field(total_words, args.words),
            format_field(total_bytes, args.bytes),
            format_field(total_chars, args.chars)
        );
    }

    Ok(())
}

fn open(fname: &str) -> Result<Box<dyn BufRead>> {
    match fname {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(fname)?))),
    }
}
