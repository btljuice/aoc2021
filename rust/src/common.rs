use std::fs::File;
use std::io::{self, BufRead};

/** @todo convert return type to Result<impl Iterator<Item=String>, {Error}> on first error */
pub fn read_lines(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|r| r.unwrap())
}
