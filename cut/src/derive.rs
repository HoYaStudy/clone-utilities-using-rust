use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(default_value = "-")]
    pub files: Vec<String>,

    #[arg(short, long, value_name = "DELIMITER", default_value = "\t")]
    pub delimiter: String,

    #[command(flatten)]
    pub extract: ArgsExtract,
}

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct ArgsExtract {
    #[arg(short, long, value_name = "FIELDS")]
    pub fields: Option<String>,

    #[arg(short, long, value_name = "BYTES")]
    pub bytes: Option<String>,

    #[arg(short, long, value_name = "CHARS")]
    pub chars: Option<String>,
}

pub fn derive_pattern() -> Args {
    Args::parse()
}
