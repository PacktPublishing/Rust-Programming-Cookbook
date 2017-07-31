//-- #########################
//-- Task: Url experiments
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 04 May 17
//-- ######################### 

#[macro_use]
extern crate error_chain;
error_chain! {
    foreign_links {
        UrlParse(url::ParseError);
    }
}

extern crate url;
use url::Url;
fn run() -> Result<()> {
    let s = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";
    let parsed = Url::parse(s)?;
    println!("The path part of the URL is: {}", parsed.path());
    Ok(())
}
quick_main!(run);


