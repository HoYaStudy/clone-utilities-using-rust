use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    pub files: Vec<String>,

    #[arg(short, long)]
    pub lines: bool,

    #[arg(short, long)]
    pub words: bool,

    #[arg(short('c'), long)]
    pub bytes: bool,

    #[arg(short('m'), long, conflicts_with("bytes"))]
    pub chars: bool,
}

pub fn derive_pattern() -> Args {
    Args::parse()
}
