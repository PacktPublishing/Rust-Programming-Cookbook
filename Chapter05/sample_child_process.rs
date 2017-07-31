//-- #########################
//-- Task: To call a child process
//-- Author: Vigneshwer.D
//-- Version: 1.0.0
//-- Date: 19 March 17
//-- #########################

// Call the standard library
use std::process::Command;

// Main execution of the code
fn main() {
    // Command to be executed
    let output = Command::new("rustc")
        .arg("--version")
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
    });

    // printing the output values
    if output.status.success() {
        let s = String::from_utf8_lossy(&output.stdout);

        print!("rustc succeeded and stdout was:\n{}", s);
    } else {
        let s = String::from_utf8_lossy(&output.stderr);

        print!("rustc failed and stderr was:\n{}", s);
    }
}