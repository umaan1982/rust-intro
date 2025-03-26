use std::io::prelude::*;
use std::fs::File;

fn log_something(filename: &'static str, string: &'static [u8; 17]) -> Result<File, std::io::Error> {
    let mut f = File::create(filename).unwrap();
    f.write_all(string)?;
    Ok(f)
}

fn main() {
    match log_something("hello.txt", b"Trying out things") {
        Ok(..) => println!("File created!"),
        Err(..) => println!("Error: could not create file.")
    }
}