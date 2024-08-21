use clap::{Arg, ArgAction, Command};

#[derive(Debug)]
pub struct Args {
    pub year: Option<i32>,
    pub month: Option<String>,
    pub show_current_year: bool,
}

pub fn builder_pattern() -> Args {
    let matches = Command::new("cal")
        .version("0.1.0")
        .author("llHoYall <hoya128@gmail.com>")
        .about("Rust version of `cal`")
        .arg(
            Arg::new("year")
                .value_name("YEAR")
                .value_parser(clap::value_parser!(i32).range(1..=9999))
                .help("Year (1-9999)"),
        )
        .arg(
            Arg::new("month")
                .value_name("MONTH")
                .short('m')
                .help("Month name or number (1-12)"),
        )
        .arg(
            Arg::new("show_current_year")
                .value_name("SHOW_YEAR")
                .short('y')
                .long("year")
                .help("Show whole current year")
                .conflicts_with_all(["month", "year"])
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    Args {
        year: matches.get_one("year").cloned(),
        month: matches.get_one("month").cloned(),
        show_current_year: matches.get_flag("show_current_year"),
    }
}
