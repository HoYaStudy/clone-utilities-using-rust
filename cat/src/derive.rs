use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    pub files: Vec<String>,

    #[arg(short('n'), long("numbe"), conflicts_with("number_nonblank_lines"))]
    pub number_lines: bool,

    #[arg(short('b'), long("number-nonblank"))]
    pub number_nonblank_lines: bool,
}

pub fn derive_pattern() -> Args {
    Args::parse()
}
