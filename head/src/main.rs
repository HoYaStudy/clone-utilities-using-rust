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
    let num_files = args.files.len();
    for (file_num, fname) in args.files.iter().enumerate() {
        match open(&fname) {
            Err(err) => eprintln!("{fname}: {err}"),
            Ok(mut file) => {
                if num_files > 1 {
                    println!("{} ==> {fname} <==", if file_num > 0 { "\n" } else { "" })
                }

                if let Some(num_bytes) = args.bytes {
                    let mut buffer = vec![0; num_bytes as usize];
                    let bytes_read = file.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..args.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{line}");
                        line.clear();
                    }
                }
            }
        };
    }
    Ok(())
}

fn open(fname: &str) -> Result<Box<dyn BufRead>> {
    match fname {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(fname)?))),
    }
}
