use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(required(true))]
    text: Vec<String>,

    #[arg(short('n'))]
    omit_newline: bool,
}

pub fn derive_pattern() {
    let args = Args::parse();
    print!(
        "{}{}",
        args.text.join(" "),
        if args.omit_newline { "" } else { "\n" }
    )
}
