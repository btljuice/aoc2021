use std::fs::read;
use itertools::Itertools;
use super::common;

pub fn part1() -> u32 {
    todo!()
}

pub fn part2() -> u32 {
    todo!()
}

pub(self) mod bingo {
    const CARD_SIZE: usize = 5;
    pub type Number = u8;
    pub type Draw = Vec<Number>;
    pub type Card = [[Number; CARD_SIZE]; CARD_SIZE];

    #[derive(Debug)]
    pub enum Error {
        DrawLineMissing,
        DrawLineInvalidFormat,
    }
}

#[cfg(test)]
pub(self) mod tests {
    use super::bingo::*;
    use super::bingo::Error::*;
    use super::common;


    fn sample() -> Result<(Draw, Vec<Card>), Error> {
        let mut lines = common::read_lines("../input/day4_sample.txt");
        let draw: Draw = lines
            .next().ok_or(DrawLineMissing)?
            .trim()
            .split(',')
            .map(|n| n.trim().parse::<Number>().map_err(|_| DrawLineInvalidFormat))
            .collect()?;

        let cards: Vec<Card> = Vec::new(); // TODO

        Ok( (draw, cards) )
    }

    #[test]
    fn test1() {
        assert_eq!(
            sample().unwrap().0,
            vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1]
        )
    }
}
