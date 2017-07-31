//-- #########################
//-- Task: File system experiments
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 28 April 17
//-- ######################### 

#[macro_use]
extern crate error_chain;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
error_chain! {
    foreign_links {
        Io(std::io::Error);
    }
}

fn run() -> Result<()> {
    let path = "lines.txt";
    let mut output = File::create(path)?;
    write!(output, "Rust\n💖\nFun")?;
    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line?);
    }
    Ok(())
}
quick_main!(run);
