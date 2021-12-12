use super::common;

pub fn part1() -> u32 {
    todo!()
}

pub fn part2() -> u32 {
    todo!()
}

pub(self) mod bingo {
    use super::common;
    use itertools::Itertools;
    use std::convert::TryFrom;
    use std::convert::TryInto;

    pub const CARD_SIZE: usize = 5;
    pub type Number = u8;
    pub type Draw = Vec<Number>;
    // Utility Wrapper to implement TryFrom,
    // because rust does not provide TryFrom implementation for arrays of arrays (e.g. `[[u8; 5]; 5]` )
    #[derive(Debug, PartialEq)]
    pub struct CardRow { pub row: [Number; CARD_SIZE] }
    impl TryFrom< Vec<Number> > for CardRow {
        type Error = super::bingo::ParseError;
        fn try_from(value: Vec<Number>) -> Result<Self, Self::Error> {
            let sz = value.len();
            value
                .try_into()
                .map_err(|_| ParseError::CardInvalidRowSize(sz))
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
        type Error = ParseError;

        fn try_from(value: Vec<CardRow>) -> Result<Self, Self::Error> {
            let sz = value.len(); // Note: Length needs to be captured right away.
            value.try_into()
                .map_err(|_| ParseError::CardInvalidNbRows(sz))
                .map(|rows| Card { rows })
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub enum ParseError {
        DrawLineMissing,
        DrawLineInvalidFormat,
        CardLineInvalidFormat,
        CardInvalidRowSize(usize),
        CardInvalidNbRows(usize),
    }

    pub fn parse_draw(line: Option<&str>) -> Result<Draw, ParseError> {
        line.ok_or(ParseError::DrawLineMissing)?
            .trim()
            .split(',')
            .map(|n| n.trim().parse::<Number>())
            .try_collect()
            .map_err(|_| ParseError::DrawLineInvalidFormat)
    }

    pub fn parse_card(lines: &[&str; CARD_SIZE]) -> Result<Card, ParseError> {
        lines.iter().map(|l| {
            let cl: Result<CardRow, ParseError> = l // `.try_into()` below requires type hint, but it does not have type parameters
                .split_whitespace()
                .map(|n| n.parse::<Number>())
                .try_collect::<_, Vec<Number>, _>()
                .map_err(|_| ParseError::CardLineInvalidFormat)?
                .try_into();
            cl
        }).try_collect::<_, Vec<_>, _>()?
          .try_into()
    }

    pub fn parse(filename: &str) -> Result<(Draw, Vec<Card>), ParseError> {
        fn chunk_to_card(chunk: impl Iterator<Item=String>) -> Result<Card, ParseError> {
            let lines: [String; CARD_SIZE] = chunk
                .skip(1)
                .collect_vec()
                .try_into().map_err(|_| ParseError::CardInvalidNbRows(0))?; // TODO
            // Well, well, we're learning about lifetimes here. Array.map converts the array.
            let slices: [&str; CARD_SIZE] = [ lines[0].as_str(), lines[1].as_str(), lines[2].as_str(), lines[3].as_str(), lines[4].as_str() ];
            parse_card(&slices)
        }

        let mut lines = common::read_lines(filename);
        let draw: Draw = parse_draw(lines.next().as_ref().map(|s| s.as_str()))?;
        let cards: Vec<Card> = lines
            .chunks(6)
            .into_iter()
            .map(chunk_to_card)
            .try_collect()?;

        Ok( (draw, cards) )
    }
}

#[cfg(test)]
pub(self) mod tests {
    use super::bingo::*;

    const CARD1_STR: &'static [&'static str; CARD_SIZE] = &[
        "22 13 17 11  0",
        "8  2 23  4 24",
        "21  9 14 16  7",
        "6 10  3 18  5",
        "1 12 20 15 19"
    ];

    const CARD1: Card = Card { rows: [
        CardRow { row: [ 22, 13, 17, 11,  0 ] },
        CardRow { row: [  8,  2, 23,  4, 24 ] },
        CardRow { row: [ 21,  9, 14, 16,  7 ] },
        CardRow { row: [  6, 10,  3, 18,  5 ] },
        CardRow { row: [  1, 12, 20, 15, 19 ] }
    ] };

    const CARD2: Card = Card { rows: [
        CardRow { row: [  3, 15,  0,  2, 22 ] },
        CardRow { row: [  9, 18, 13, 17,  5 ] },
        CardRow { row: [ 19,  8,  7, 25, 23 ] },
        CardRow { row: [ 20, 11, 10, 24,  4 ] },
        CardRow { row: [ 14, 21, 16, 12,  6 ] }
    ] };

    const CARD3: Card = Card { rows: [
        CardRow { row: [ 14, 21, 17, 24,  4 ] },
        CardRow { row: [ 10, 16, 15,  9, 19 ] },
        CardRow { row: [ 18,  8, 23, 26, 20 ] },
        CardRow { row: [ 22, 11, 13,  6,  5 ] },
        CardRow { row: [  2,  0, 12,  3,  7 ] }
    ] };

    fn sample() -> Result<(Draw, Vec<Card>), ParseError> { parse("../input/day4_sample.txt") }

    #[test]
    fn test_parse_card() {
        let actual = parse_card(CARD1_STR).unwrap();
        assert_eq!(actual, CARD1)
    }

    #[test]
    fn test1() {
        let (draw, cards) = sample().unwrap();
        assert_eq!(
            draw,
            vec![7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1]
        );
        assert_eq!(cards, vec![CARD1, CARD2, CARD3]);
    }
}
