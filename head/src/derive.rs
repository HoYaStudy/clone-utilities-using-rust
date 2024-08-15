use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    pub files: Vec<String>,

    #[arg(short('n'), long, default_value = "10", value_name="LINES", value_parser=clap::value_parser!(u64).range(1..))]
    pub lines: u64,

    #[arg(short('c'), long, value_name="BYTES", conflicts_with("lines"),value_parser=clap::value_parser!(u64).range(1..))]
    pub bytes: Option<u64>,
}

pub fn derive_pattern() -> Args {
    Args::parse()
}
