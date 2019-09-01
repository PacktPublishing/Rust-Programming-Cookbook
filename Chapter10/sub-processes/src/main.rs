use std::error::Error;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let mut ls_child = Command::new("ls");
    if !cfg!(target_os = "windows") {
        ls_child.args(&["-alh"]);
    }
    println!("{}", ls_child.status()?);
    ls_child.current_dir("src/");
    println!("{}", ls_child.status()?);

    let env_child = Command::new("env")
        .env("CANARY", "0x5ff")
        .stdout(Stdio::piped())
        .spawn()?;

    let env_output = &env_child.wait_with_output()?;
    let canary = String::from_utf8_lossy(&env_output.stdout)
        .split_ascii_whitespace()
        .filter(|line| *line == "CANARY=0x5ff")
        .count();

    // found it!
    assert_eq!(canary, 1);

    let mut rev_child = Command::new("rev")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    {
        rev_child
            .stdin
            .as_mut()
            .expect("Could not open stdin")
            .write_all(b"0x5ff")?;
    }

    let output = rev_child.wait_with_output()?;
    assert_eq!(String::from_utf8_lossy(&output.stdout), "ff5x0");

    Ok(())
}
