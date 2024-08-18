use clap::{Arg, Command};

#[derive(Debug)]
pub struct Args {
    pub files: Vec<String>,
    pub delimiter: String,
    pub extract: ArgsExtract,
}

#[derive(Debug)]
struct ArgsExtract {
    fields: Option<String>,
    bytes: Option<String>,
    chars: Option<String>,
}

pub fn builder_pattern() -> Args {
    let matches = Command::new("cut")
        .version("0.1.0")
        .author("llHoYall <hoya128@gmail.com>")
        .about("Rust version of `cut`")
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("Input file(s)")
                .num_args(0..)
                .default_value("-"),
        )
        .arg(
            Arg::new("delimiter")
                .value_name("DELIMITER")
                .short('d')
                .long("delim")
                .help("Field delimiter")
                .default_value("\t"),
        )
        .arg(
            Arg::new("fields")
                .value_name("FIELDS")
                .short('f')
                .long("fields")
                .help("Selected fields"),
        )
        .arg(
            Arg::new("bytes")
                .value_name("BYTES")
                .short('b')
                .long("bytes")
                .help("Selected bytes"),
        )
        .arg(
            Arg::new("chars")
                .value_name("CHARS")
                .short('c')
                .long("chars")
                .help("Selected characters"),
        )
        .group(
            ArgGroup::new("extract")
                .args(["fields", "bytes", "chars"])
                .required(true)
                .multiple(false),
        )
        .get_matches();

    Args {
        files: matches.get_many("files").unwrap().cloned().collect(),
        delimiter: matches.get_one("delimiter").cloned().unwrap(),
        extract: ArgsExtract {
            fields: matches.get_one("fields").cloned(),
            bytes: matches.get_one("bytes").cloned(),
            chars: matches.get_one("chars").cloned(),
        },
    }
}
