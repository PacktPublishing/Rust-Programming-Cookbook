use glob;
use std::error::Error;
use std::io;
use std::path::{Path, PathBuf};

type GenericError = Box<dyn Error + Send + Sync + 'static>;

fn walk(dir: &Path, cb: &dyn Fn(&PathBuf), recurse: bool) -> io::Result<()> {
    for entry in dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if recurse && path.is_dir() {
            walk(&path, cb, true)?;
        }
        cb(&path);
    }
    Ok(())
}

fn walk_glob(pattern: &str, cb: &dyn Fn(&PathBuf)) -> Result<(), GenericError> {
    for entry in glob::glob(pattern)? {
        cb(&entry?);
    }
    Ok(())
}

fn main() -> Result<(), GenericError> {
    let path = Path::new("./src");
    println!("Listing '{}'", path.display());
    println!("===");
    walk(path, &|d| println!("  {}", d.display()), true)?;
    println!();

    let glob_pattern = "../**/*.rs";
    println!("Listing by glob filter: {}", glob_pattern);
    println!("===");
    walk_glob(glob_pattern, &|d| println!("  {}", d.display()))?;
    println!();

    let glob_pattern = "Cargo.*";
    println!("Listing by glob filter: {}", glob_pattern);
    println!("===");
    walk_glob(glob_pattern, &|d| println!("  {}", d.display()))?;
    Ok(())
}
