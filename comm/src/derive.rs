use clap::{ArgAction, Parser};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg()]
    pub file1: String,

    #[arg()]
    pub file2: String,

    #[arg(short('1'), action(ArgAction::SetFalse))]
    pub show_col1: bool,

    #[arg(short('2'), action(ArgAction::SetFalse))]
    pub show_col2: bool,

    #[arg(short('3'), action(ArgAction::SetFalse))]
    pub show_col3: bool,

    #[arg(short)]
    pub insensitive: bool,

    #[arg(short, long("output-delimiter"), default_value = "\t")]
    pub delimiter: String,
}

pub fn derive_pattern() -> Args {
    Args::parse()
}
