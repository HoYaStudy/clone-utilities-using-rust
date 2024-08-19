use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(required = true)]
    pub files: Vec<String>,

    #[arg(value_name = "LINES", short('n'), long, default_value = "10")]
    pub lines: String,

    #[arg(value_name = "BYTES", short('c'), long, conflicts_with("lines"))]
    pub bytes: Option<String>,

    #[arg(short, long)]
    pub quiet: bool,
}

pub fn derive_pattern() -> Args {
    Args::parse()
}
