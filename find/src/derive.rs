use clap::{ArgAction, Parser};
use regex::Regex;

use crate::EntryType;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(value_name = "PATH", default_value = ".")]
    pub paths: Vec<String>,

    #[arg(short('n'), long("name"), value_name="NAME", value_parser(Regex::new), action(ArgAction::Append), num_args(0..))]
    pub names: Vec<Regex>,

    #[arg(short('t'), long("type"), value_name="TYPE", value_parser(clap::value_parser!(EntryType)), action(ArgAction::Append), num_args(0..))]
    pub entry_types: Vec<EntryType>,
}

pub fn derive_pattern() -> Args {
    Args::parse()
}
