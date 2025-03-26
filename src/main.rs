
#[macro_use] extern crate nickel;
extern crate chrono;

use std::io::prelude::*;
use tokio; 
use std::fs::{OpenOptions};
use std::io;
use chrono::*;
use nickel::Nickel;

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

fn log_time(filename: &'static str) -> io::Result<String> {
    let entry = formatted_time_entry();
    let bytes = entry.as_bytes();

    (record_entry_in_log(filename, &bytes))?;
    Ok(entry)
}

fn do_log_time() -> String {
    match log_time("log.txt") {
        Ok(entry) => format!("File created! {}", entry),
        Err(e) => format!("Error: {}", e)
    }
}

async fn listen_server(server: Nickel) {
    server.listen("127.0.0.1:6767").await.expect("Failed to start server");
}


fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            do_log_time()
        }
    });

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(listen_server(server));
}