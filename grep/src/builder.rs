use clap::{Arg, ArgAction, Command};

#[derive(Debug)]
pub struct Args {
    pub pattern: String,
    pub files: Vec<String>,
    pub insensitive: bool,
    pub recursive: bool,
    pub count: bool,
    pub invert: bool,
}

pub fn builder_pattern() -> Args {
    let matches = Command::new("grep")
        .version("0.1.0")
        .author("llHoYall <hoya128@gmail.com>")
        .about("Rust version of `grep`")
        .arg(
            Arg::new("pattern")
                .value_name("PATTERN")
                .help("Search pattern")
                .required(true),
        )
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input file(s)")
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("insensitive")
                .short('i')
                .long("insensitive")
                .help("Case-insensitive")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("recursive")
                .short('r')
                .long("recursive")
                .help("Recursive search")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .help("Count occurrences")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("invert")
                .short('v')
                .long("invert-match")
                .help("Invert match")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    Args {
        pattern: matches.get_one("pattern").cloned().unwrap(),
        files: matches.get_many("files").unwrap().cloned().collect(),
        insensitive: matches.get_flag("insensitive"),
        recursive: matches.get_flag("recursive"),
        count: matches.get_flag("count"),
        invert: matches.get_flag("invert"),
    }
}
