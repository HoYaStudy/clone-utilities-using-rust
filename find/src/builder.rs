use clap::{Arg, ArgAction, Command};
use regex::Regex;

use find::EntryType;

#[derive(Debug)]
pub struct Args {
    pub paths: Vec<String>,
    pub names: Vec<Regex>,
    pub entry_types: Vec<EntryType>,
}

pub fn builder_pattern() -> Args {
    let matches = Command::new("find")
        .version("0.1.0")
        .author("llHoYall <hoya128@gmail.com>")
        .about("Rust version of `find`")
        .arg(
            Arg::new("paths")
                .value_name("PATH")
                .help("Search paths")
                .default_value(".")
                .num_args(0..),
        )
        .arg(
            Arg::new("names")
                .value_name("NAME")
                .short('n')
                .long("name")
                .help("Name")
                .value_parser(Regex::new)
                .action(ArgAction::Append)
                .num_args(0..),
        )
        .arg(
            Arg::new("types")
                .value_name("TYPE")
                .short('t')
                .long("type")
                .help("Entry type")
                .value_parser(clap::value_parser!(EntryType))
                .action(ArgAction::Append)
                .num_args(0..),
        )
        .get_matches();

    Args {
        paths: matches.get_many("paths").unwrap().cloned().collect(),
        names: matches
            .get_many("names")
            .unwrap_or_default()
            .cloned()
            .collect(),
        entry_types: matches
            .get_many("types")
            .unwrap_or_default()
            .cloned()
            .collect(),
    }
}
