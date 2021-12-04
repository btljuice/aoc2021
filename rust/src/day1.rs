use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use itertools::Itertools;

pub fn day1_part1() -> u32 {
    count_increases(depths())
}

pub fn day1_part2() -> u32 {
    let rolling_sums: Vec<u32> = depths()
        .collect::<Vec<u32>>()
        .windows(3)
        .filter_map(|w| match w {
            [a, b, c] => Some(a + b + c),
            _ => None,
        })
        .collect();

    count_increases(rolling_sums.into_iter())
}

fn count_increases(depths: impl Iterator<Item = u32>) -> u32 {
    depths
        .tuple_windows()
        .fold(0, |n, (d0, d1)| n + u32::from(d1 > d0))
}

/** @todo Need to understand what a Box is. (Seems to describe a heap allocated element). */
fn depths() -> impl Iterator<Item = u32> {
    read_lines("../input/day1.txt")
        .filter_map(|l| l.ok())
        .map(|s| s.parse::<u32>().unwrap())
        .into_iter()
}

fn read_lines<P>(filename: P) -> impl Iterator<Item = Result<String, std::io::Error>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}
