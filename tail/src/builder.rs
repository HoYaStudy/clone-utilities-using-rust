use clap::{Arg, ArgAction, Command};

#[derive(Debug)]
pub struct Args {
    pub files: Vec<String>,
    pub lines: String,
    pub bytes: Option<String>,
    pub quiet: bool,
}

pub fn builder_pattern() -> Args {
    let matches = Command::new("tail")
        .version("0.1.0")
        .author("llHoYall <hoya128@gmail.com>")
        .about("Rust version of `tail`")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .conflicts_with("lines")
                .help("Number of bytes"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .action(ArgAction::SetTrue)
                .help("Suppress headers"),
        )
        .get_matches();

    Args {
        files: matches.get_many("files").unwrap().cloned().collect(),
        lines: matches.get_one("lines").cloned().unwrap(),
        bytes: matches.get_one("bytes").cloned(),
        quiet: matches.get_flag("quiet"),
    }
}
