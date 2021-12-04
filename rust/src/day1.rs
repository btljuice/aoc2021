// `.tuple_windows() can be used on iterators, while .windows() is only available on slices
use itertools::Itertools;
use super::common;

pub fn day1_part1() -> u32 {
    count_increases(depths())
}

pub fn day1_part2() -> u32 {
    let rolling_sums = depths()
        .tuple_windows::<(_, _, _)>()
        .map(|(a, b, c)| a + b + c);

    count_increases(rolling_sums)
}

fn count_increases(depths: impl Iterator<Item = u32>) -> u32 {
    depths
        .tuple_windows()
        .fold(0, |n, (d0, d1)| n + u32::from(d1 > d0))
}

fn depths() -> impl Iterator<Item = u32> {
    common::read_lines("../input/day1.txt")
        .map(|s| s.parse::<u32>().unwrap())
}
