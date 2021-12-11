use std::fs::read;
use std::convert::TryInto;
use itertools::Itertools;
use super::common;

pub fn part1() -> u32 {
    todo!()
}

pub fn part2() -> u32 {
    todo!()
}

pub(self) mod bingo {
    use itertools::Itertools;
    use std::convert::{TryFrom, TryInto};

    pub const CARD_SIZE: usize = 5;
    pub type Number = u8;
    pub type Draw = Vec<Number>;
    // Utility Wrapper to implement TryFrom,
    // because rust does not provide TryFrom implementation for arrays of arrays (e.g. `[[u8; 5]; 5]` )
    #[derive(Debug, PartialEq)]
    pub struct CardRow { pub row: [Number; CARD_SIZE] }
    impl TryFrom< Vec<Number> > for CardRow {
        type Error = super::bingo::BingoError;
        fn try_from(value: Vec<Number>) -> Result<Self, Self::Error> {
            let sz = value.len();
            value
                .try_into()
                .map_err(|_| BingoError::CardInvalidRowSize(sz))
                .map(|row| CardRow { row })
        }
    }

    // CLEAN: I'd ideally swap this for a matrix or multidimensional array type such as:
    //      Array2: https://docs.rs/ndarray/0.12.1/ndarray/struct.ArrayBase.html#array
    //      But I want to toy w/ the primitive types for the moment.
    // ANSME: How to assess a crate's package adoption + maturity, apart looking at github repo?
    #[derive(Debug, PartialEq)]
    pub struct Card { pub rows: [CardRow; CARD_SIZE] }
    impl TryFrom< Vec<CardRow> > for Card {
        type Error = BingoError;

        fn try_from(value: Vec<CardRow>) -> Result<Self, Self::Error> {
            let sz = value.len(); // Note: Length needs to be captured right away.
            value.try_into()
                .map_err(|_| BingoError::CardInvalidNbRows(sz))
                .map(|rows| Card { rows })
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub enum BingoError {
        DrawLineMissing,
        DrawLineInvalidFormat,
        CardLineInvalidFormat,
        CardInvalidRowSize(usize),
        CardInvalidNbRows(usize),
    }

    pub fn parse_draw(line: Option<&str>) -> Result<Draw, BingoError> {
        line.ok_or(BingoError::DrawLineMissing)?
            .trim()
            .split(',')
            .map(|n| n.trim().parse::<Number>())
            .try_collect()
            .map_err(|_| BingoError::DrawLineInvalidFormat)
    }

    pub fn parse_card(lines: &[&str; CARD_SIZE]) -> Result<Card, BingoError> {
        lines.iter().map(|l| {
            let cl: Result<CardRow, BingoError> = l // `.try_into()` below requires type hint, but it does not have type parameters
                .split_whitespace()
                .map(|n| n.parse::<Number>())
                .try_collect::<_, Vec<Number>, _>()
                .map_err(|_| BingoError::CardLineInvalidFormat)?
                .try_into();
            cl
        }).try_collect::<_, Vec<_>, _>()?
          .try_into()
    }
}

#[cfg(test)]
pub(self) mod tests {
    use super::bingo::*;
    use super::common;

    const CARD: &'static [&'static str; CARD_SIZE] = &[
        " 1  2  3  4  5",
        " 6  7  8  9 10",
        "11 12 13 14 15",
        "16 17 18 19 20",
        "21 22 23 24 25"
    ];

    #[test]
    fn test_parse_card() {
        let actual = parse_card(CARD).unwrap();
        let expected: Card = Card { rows: [
            CardRow { row: [ 1,  2,  3,  4,  5] },
            CardRow { row: [ 6,  7,  8,  9, 10] },
            CardRow { row: [11, 12, 13, 14, 15] },
            CardRow { row: [16, 17, 18, 19, 20] },
            CardRow { row: [21, 22, 23, 24, 25] }
        ] };
        assert_eq!(actual, expected)
    }


    fn sample() -> Result<(Draw, Vec<Card>), BingoError> {
        let mut lines = common::read_lines("../input/day4_sample.txt");
        let draw: Draw = parse_draw(lines.next().as_ref().map(|s| s.as_str()))?;

        let cards: Vec<Card> = Vec::new();

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
