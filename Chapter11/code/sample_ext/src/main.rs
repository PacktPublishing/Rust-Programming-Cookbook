//-- #########################
//-- Task: glob experiments
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 04 May 17
//-- ######################### 

#[macro_use]
extern crate error_chain;
error_chain! {
    foreign_links {
        Glob(glob::GlobError);
        Pattern(glob::PatternError);
    }
}

extern crate glob;
use glob::glob;
fn run() -> Result<()> {
    for entry in glob("**/*.png")? {
        println!("{}", entry?.display());
    }

    Ok(())
}
quick_main!(run);