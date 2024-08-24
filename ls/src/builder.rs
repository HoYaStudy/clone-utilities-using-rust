use clap::{Arg, ArgAction, Command};

#[derive(Debug)]
pub struct Args {
    pub paths: Vec<String>,
    pub long: bool,
    pub show_hidden: bool,
}

pub fn builder_pattern() -> Args {
    let matches = Command::new("ls")
        .version("0.1.0")
        .author("llHoYall <hoya128@gmail.com>")
        .about("Rust version of `ls`")
        .arg(
            Arg::new("paths")
                .value_name("PATH")
                .help("Files and/or directories")
                .default_value(".")
                .num_args(0..),
        )
        .arg(
            Arg::new("long")
                .short('l')
                .long("long")
                .help("Long listing")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .action(ArgAction::SetTrue)
                .help("Show all files"),
        )
        .get_matches();

    Args {
        paths: matches.get_many("paths").unwrap().cloned().collect(),
        long: matches.get_flag("long"),
        show_hidden: matches.get_flag("all"),
    }
}
