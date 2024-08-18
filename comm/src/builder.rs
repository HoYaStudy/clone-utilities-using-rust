use clap::{Arg, ArgAction, Command};

#[derive(Debug)]
pub struct Args {
    pub file1: String,
    pub file2: String,
    pub show_col1: bool,
    pub show_col2: bool,
    pub show_col3: bool,
    pub insensitive: bool,
    pub delimiter: String,
}

pub fn builder_pattern() -> Args {
    let matches = Command::new("comm")
        .version("0.1.0")
        .author("llHoYall <hoya128@gmail.com>")
        .about("Rust version of `comm`")
        .arg(
            Arg::new("file1")
                .value_name("FILE1")
                .help("Input file 1")
                .required(true),
        )
        .arg(
            Arg::new("file2")
                .value_name("FILE2")
                .help("Input file 2")
                .required(true),
        )
        .arg(
            Arg::new("suppress_col1")
                .short('1')
                .action(ArgAction::SetTrue)
                .help("Suppress printing of column 1"),
        )
        .arg(
            Arg::new("suppress_col2")
                .short('2')
                .action(ArgAction::SetTrue)
                .help("Suppress printing of column 2"),
        )
        .arg(
            Arg::new("suppress_col3")
                .short('3')
                .action(ArgAction::SetTrue)
                .help("Suppress printing of column 3"),
        )
        .arg(
            Arg::new("insensitive")
                .short('i')
                .action(ArgAction::SetTrue)
                .help("Case-insensitive comparison of lines"),
        )
        .arg(
            Arg::new("delimiter")
                .short('d')
                .long("output-delimiter")
                .value_name("DELIM")
                .help("Output delimiter")
                .default_value("\t"),
        )
        .get_matches();

    Args {
        file1: matches.get_one("file1").cloned().unwrap(),
        file2: matches.get_one("file2").cloned().unwrap(),
        show_col1: !matches.get_flag("suppress_col1"),
        show_col2: !matches.get_flag("suppress_col2"),
        show_col3: !matches.get_flag("suppress_col3"),
        insensitive: matches.get_flag("insensitive"),
        delimiter: matches.get_one("delimiter").cloned().unwrap(),
    }
}
