use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg()]
    pub pattern: String,

    #[arg(value_name = "FILE", default_value = "-")]
    pub files: Vec<String>,

    #[arg(short, long)]
    pub insensitive: bool,

    #[arg(short, long)]
    pub recursive: bool,

    #[arg(short, long)]
    pub count: bool,

    #[arg(short('v'), long("invert-match"))]
    pub invert: bool,
}

pub fn derive_pattern() -> Args {
    Args::parse()
}
