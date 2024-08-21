use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[arg(value_parser(clap::value_parser!(i32).range(1..=9999)))]
    pub year: Option<i32>,

    #[arg(short)]
    pub month: Option<String>,

    #[arg(short('y'), long("year"), conflicts_with_all(["year", "month"]))]
    pub show_current_year: bool,
}

pub fn derive_pattern() -> Args {
    Args::parse()
}
