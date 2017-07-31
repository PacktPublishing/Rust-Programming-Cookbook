//-- #########################
//-- Task: Encoding experiments
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 04 May 17
//-- ######################### 

#[macro_use]
extern crate error_chain;
error_chain! {
    foreign_links {
        Json(serde_json::Error);
    }
}

#[macro_use]
extern crate serde_json;
use serde_json::Value;
fn run() -> Result<()> {
    let j = r#"{
                 "userid": 103609,
                 "verified": true,
                 "access_privileges": [
                   "user",
                   "admin"
                 ]
               }"#;
    let parsed: Value = serde_json::from_str(j)?;
    let expected = json!({
        "userid": 103609,
        "verified": true,
        "access_privileges": [
            "user",
            "admin"
        ]
    });
    assert_eq!(parsed, expected);
    Ok(())
}
quick_main!(run);
