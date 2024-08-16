use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(value_name = "IN_FILE", default_value = "-")]
    pub in_file: String,

    #[arg(value_name = "OUT_FILE")]
    pub out_file: Option<String>,

    #[arg(short, long)]
    pub count: bool,
}

pub fn derive_pattern() -> Args {
    Args::parse()
}
