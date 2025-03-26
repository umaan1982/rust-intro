extern crate chrono;

use std::io::prelude::*;
use std::fs::{OpenOptions};
use std::io;
use chrono::*;

fn formatted_time_entry() -> String {
    let local: DateTime<Local> = Local::now();
    let formatted = local.format("%a, %b %d %Y %I:%M:%S %p\n").to_string();
    formatted
}

fn record_entry_in_log(filename: &str, bytes: &[u8]) -> io::Result<()> {
    let mut f = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(filename)?;
    (f.write_all(bytes))?;
    Ok(())
}

fn log_time(filename: &'static str) -> io::Result<()> {
    let entry = formatted_time_entry();
    let bytes = entry.as_bytes();

    (record_entry_in_log(filename, &bytes))?;
    Ok(())
}

fn main() {
    match log_time("log.txt") {
        Ok(..) => println!("File created!"),
        Err(..) => println!("Error: could not create file.")
    }
}