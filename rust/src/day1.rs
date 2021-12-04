use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn day1_part1() -> u32 {
    count_increases(&depths().collect::<Vec<u32>>())
}

pub fn day1_part2() -> u32 {
    let rolling_sums: Vec<u32> = depths()
        .collect::<Vec<u32>>()
        .windows(3)
        .filter_map(|w| match w {
            [a, b, c] => Some(a + b + c),
            _ => None
        }).collect();

    count_increases(&rolling_sums)
}

fn count_increases(depths: &Vec<u32>) -> u32 {
    depths
        .windows(2)
        .filter_map(|w| match w {
            [a, b] => Some((a, b)),
            _ => None
        })
        .fold(0, |n, (d0, d1)| n + u32::from(d1 > d0))
}

/** @todo Need to understand what a Box is. (Seems to describe a heap allocated element). */
fn depths() -> Box<dyn Iterator<Item=u32>> {
    Box::new(
        read_lines("../input/day1.txt")
            .filter_map(|l| l.ok())
            .map(|s| s.parse::<u32>().unwrap()).into_iter()
    )
}

fn read_lines<P>(filename: P) -> impl Iterator<Item = Result<String, std::io::Error>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines()
}
