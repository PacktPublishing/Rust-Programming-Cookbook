use clap::{App, Arg, SubCommand};
use std::fs::DirEntry;
use std::path::Path;

use std::io;

struct Exclusion(String);

impl Exclusion {
    pub fn is_excluded(&self, path: &Path) -> bool {
        path.file_name()
            .map_or(false, |f| f.to_string_lossy().find(&self.0).is_some())
    }
}

fn walk(
    dir: &Path,
    exclusion: &Option<Exclusion>,
    cb: &dyn Fn(&DirEntry),
    recurse: bool,
) -> io::Result<()> {
    for entry in dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if !exclusion.as_ref().map_or(false, |e| e.is_excluded(&path)) {
            if recurse && path.is_dir() {
                walk(&path, exclusion, cb, true)?;
            }
            cb(&entry);
        }
    }
    Ok(())
}

fn print_if_file(entry: &DirEntry) {
    let path = entry.path();
    if !path.is_dir() {
        println!("{}", path.to_string_lossy())
    }
}
fn print_if_dir(entry: &DirEntry) {
    let path = entry.path();
    if path.is_dir() {
        println!("{}", path.to_string_lossy())
    }
}

fn main() -> io::Result<()> {
    let matches = App::new("list")
        .version("1.0")
        .author("Claus M - claus.matzinger+kb@gmail.com")
        .about("")
        .arg(
            Arg::with_name("exclude")
                .short("e")
                .long("exclude")
                .value_name("NAME")
                .help("Exclude directories/files with this name")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("recursive")
                .short("r")
                .long("recursive")
                .help("Recursively descend into subdirectories"),
        )
        .subcommand(
            SubCommand::with_name("files")
                .about("Lists files only")
                .arg(
                    Arg::with_name("PATH")
                        .help("The path to start looking")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("dirs")
                .about("Lists directories only")
                .arg(
                    Arg::with_name("PATH")
                        .help("The path to start looking")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    let recurse = matches.is_present("recursive");
    let exclusions = matches.value_of("exclude").map(|e| Exclusion(e.into()));

    match matches.subcommand() {
        ("files", Some(subcmd)) => {
            let path = Path::new(subcmd.value_of("PATH").unwrap());
            walk(path, &exclusions, &print_if_file, recurse)?;
        }
        ("dirs", Some(subcmd)) => {
            let path = Path::new(subcmd.value_of("PATH").unwrap());
            walk(path, &exclusions, &print_if_dir, recurse)?;
        }
        _ => {}
    }
    Ok(())
}
