mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

fn main() {
    println!("day1-1 answer = {}", day1::part1());
    println!("day1-2 answer = {}", day1::part2());
    println!("day2-1 answer = {}", day2::part1());
    println!("day2-2 answer = {}", day2::part2());
    println!("day3-1 answer = {}", day3::part1());
    println!("day3-2 answer = {}", day3::part2());
    println!("day4-1 answer = {}", day4::part1());
    println!("day4-2 answer = {}", day4::part2());
    println!("day5-1 answer = {}", day5::part1());
    println!("day5-2 answer = {}", day5::part2());
    day6::part1_and_2();
}
