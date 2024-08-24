use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(default_value = ".")]
    pub paths: Vec<String>,

    #[arg(short, long)]
    pub long: bool,

    #[arg(short('a'), long("all"))]
    pub show_hidden: bool,
}

pub fn derive_pattern() -> Args {
    Args::parse()
}
