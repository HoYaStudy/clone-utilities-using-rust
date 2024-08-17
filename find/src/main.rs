use anyhow::Result;
use walkdir::{DirEntry, WalkDir};

use find::EntryType;

// mod builder;
mod derive;

fn main() {
    // if let Err(e) = run(builder::builder_pattern()) {
    if let Err(e) = run(derive::derive_pattern()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

// fn run(args: builder::Args) -> Result<()> {
fn run(args: derive::Args) -> Result<()> {
    let type_filter = |entry: &DirEntry| {
        args.entry_types.is_empty()
            || args.entry_types.iter().any(|entry_type| match entry_type {
                EntryType::Dir => entry.file_type().is_dir(),
                EntryType::File => entry.file_type().is_file(),
                EntryType::Link => entry.file_type().is_symlink(),
            })
    };

    let name_filter = |entry: &DirEntry| {
        args.names.is_empty()
            || args
                .names
                .iter()
                .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in args.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{e}");
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();
        println!("{}", entries.join("\n"));
    }
    Ok(())
}
