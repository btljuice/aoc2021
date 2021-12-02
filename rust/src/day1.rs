use std::fs::File;
use std::io::{self, BufRead, Lines, BufReader};
use std::path::Path;

pub fn day1() -> u32 {
   read_lines("../input/day1.txt")
       .filter_map(|l| l.ok())
       .map(|s| s.parse::<u32>().unwrap())
       .fold((u32::MAX, 0), | (i0, n), i1 | (i1, n + u32::from(i1 > i0)))
       .1
}

fn read_lines<P>(filename: P) -> Lines<BufReader<File>> where P: AsRef<Path> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}
