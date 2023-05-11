#[path = "../example_1.rs"]
mod example_1;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;

#[derive(Debug)]
enum ReadError {
    Io(std::io::Error),
    ParseInt(ParseIntError),
}

impl From<std::io::Error> for ReadError {
    fn from(e: std::io::Error) -> Self { ReadError::Io(e) }
}

impl From<ParseIntError> for ReadError {
    fn from(e: ParseIntError) -> Self { ReadError::ParseInt(e) }
}

fn main() -> Result<(), ReadError> {
    let mut args = std::env::args().skip(1);
    let input_filename = args.next().expect("Need input filename as arg 1");
    let input_key = args.next().expect("Need key as arg 2");
    let input_key: u64 = input_key.parse()?;
    let input_file = File::open(input_filename)?;
    let buffered = BufReader::new(input_file);

    let mut data: Vec<u64> = Vec::new();
    for (_j, line) in buffered.lines().enumerate() {
        let line = line?;
        // println!("line[{_j}]: {line}");
        data.push(line.parse()?);
    }

    let result = example_1::search(input_key, &data);
    match result {
        Ok(found) => println!("Found key {input_key}, {found:?}"),
        Err(unfound) => println!("Missing key {input_key}; could insert at index {}",
                                 unfound.insert_at),
    }

    Ok(())
}
