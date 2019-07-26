use std::fs;
use std::io;

fn print_file_contents_qm(filename: &str) -> Result<(), io::Error> {
    let contents = fs::read_to_string(filename)?;
    println!("File contents, external fn: {:?}", contents);
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    println!("Ok: {:?}", print_file_contents_qm("testfile.txt"));
    println!("Err: {:?}", print_file_contents_qm("not-a-file"));
    
    let contents = fs::read_to_string("testfile.txt")?;
    println!("File contents, main fn: {:?}", contents);
    Ok(())
}
