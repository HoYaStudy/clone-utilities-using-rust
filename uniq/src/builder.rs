use clap::{Arg, ArgAction, Command};

#[derive(Debug)]
pub struct Args {
    pub in_file: String,
    pub out_file: Option<String>,
    pub count: bool,
}

pub fn builder_pattern() -> Args {
    let matches = Command::new("uniq")
        .version("0.1.0")
        .author("llHoYall <hoya128@gmail.com>")
        .about("Rust version of `uniq`")
        .arg(
            Arg::new("in_file")
                .value_name("IN_FILE")
                .help("Input file")
                .default_value("-"),
        )
        .arg(
            Arg::new("out_file")
                .value_name("OUT_FILE")
                .help("Output file"),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .action(ArgAction::SetTrue)
                .help("Show counts"),
        )
        .get_matches();

    Args {
        in_file: matches.get_one("in_file").cloned().unwrap(),
        out_file: matches.get_one("out_file").cloned(),
        count: matches.get_flag("count"),
    }
}
