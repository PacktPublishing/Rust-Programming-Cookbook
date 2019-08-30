use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Seek, Write};
use std::path::Path;

const TEST_FILE_NAME: &str = "lorem.txt";

fn read() -> io::Result<()> {
    let path = Path::new(TEST_FILE_NAME);

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let words: Vec<usize> = buffered
        .lines()
        .map(|line| line.unwrap().split_ascii_whitespace().count())
        .collect();
    let avg_word_count = words.iter().sum::<usize>() as f32 / words.len() as f32;
    println!(
        "{}: Average words per line: {:.2}",
        path.to_string_lossy(),
        avg_word_count
    );

    let mut input = File::open(path)?;
    let mut input_buffer = String::new();
    input.read_to_string(&mut input_buffer)?;

    // ... or ...

    let lorem = fs::read_to_string(path)?;
    println!(
        "{}: Length in characters : {}",
        path.to_string_lossy(),
        lorem.len()
    );

    input.seek(io::SeekFrom::Start(0))?; // reset file pointer to the beginning

    println!(
        "{}: Length in bytes: {}",
        path.to_string_lossy(),
        input.bytes().count()
    );
    Ok(())
}

fn write() -> io::Result<()> {
    let mut path = Path::new(".").to_path_buf();

    path.push("hello.txt");

    let mut file = File::create(path)?;
    println!("Opened {:?}", file.metadata()?);

    file.write_all(b"Hello")?;

    let mut buffered = BufWriter::new(file);
    write!(buffered, " World!")?;
    write!(buffered, "\n{: >width$}", width=0x5ff)?;
    Ok(())
}

fn main() -> io::Result<()> {
    println!("===== READ =====");
    read()?;
    println!();
    println!("===== WRITE ====");
    write()?;
    Ok(())
}
