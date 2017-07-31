//-- #########################
//-- Task: tar experiments
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 04 May 17
//-- ######################### 

#[macro_use]
extern crate error_chain;
error_chain! {
    foreign_links {
        Io(std::io::Error);
    }
}

extern crate flate2;
extern crate tar;
use std::fs::File;
use flate2::read::GzDecoder;
use tar::Archive;
fn run() -> Result<()> {
    let path = "archive.tar.gz";
    // Open a compressed tarball
    let tar_gz = File::open(path)?;
    // Decompress it
    let tar = GzDecoder::new(tar_gz)?;
    // Load the archive from the tarball
    let mut archive = Archive::new(tar);
    // Unpack the archive inside curent working directory
    archive.unpack(".")?;
    Ok(())
}
quick_main!(run);
