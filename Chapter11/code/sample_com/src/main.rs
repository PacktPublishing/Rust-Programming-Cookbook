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

extern crate tar;
extern crate flate2;
use std::fs::File;
use flate2::Compression;
use flate2::write::GzEncoder;
fn run() -> Result<()> {
    let tar_gz = File::create("archive.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::Default);
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("./backup", "../sample_com")?;
    Ok(())
}
quick_main!(run);