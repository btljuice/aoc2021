use lazy_regex::regex_captures;
use shrinkwraprs::Shrinkwrap;
use std::collections::HashSet;
use std::convert::AsRef;
use std::fmt::Display;
use std::path::Path;

use crate::common;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
struct Pos(i32, i32);
impl Pos {
  fn from_str(s: impl AsRef<str>) -> Self {
    let (x, y) = s.as_ref().split_once(',').unwrap();
    Pos(x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
  }

  fn max(&self, &Pos(i1, j1): &Self) -> Self {
    let &Pos(i0, j0) = self;
    Pos(i0.max(i1), j0.max(j1))
  }
}

#[derive(PartialEq, Eq, Debug)]
enum Instruction {
  X(i32),
  Y(i32)
}
impl Instruction {
  fn from_str(s: impl AsRef<str>) -> Self {
    let (_, axis, value) = regex_captures!(r"^fold along ([x,y])=(\d+)$", s.as_ref()).unwrap();
    let value = value.parse::<i32>().unwrap();
    match axis {
      "x" => Instruction::X(value),
      "y" => Instruction::Y(value),
      _ => panic!("Unexpected. Axis should be either x or y"),
    }
  }

  fn fold<I>(&self, dots: I) -> HashSet<Pos> where I: IntoIterator<Item=Pos> {
    match self {
      &Instruction::Y(row) => dots
        .into_iter()
        .map(|Pos(i, j)| if j > row { Pos(i, 2*row - j) } else { Pos(i, j) } )
        .inspect(|&Pos(i, j)| assert!(i >= 0 && j >= 0, "Fold should always produce positive values") )
        .collect(),
      &Instruction::X(col) => dots
        .into_iter()
        .map(|Pos(i, j)| if i > col { Pos(2*col - i, j) } else { Pos(i, j) } )
        .inspect(|&Pos(i, j)| assert!(i >= 0 && j >= 0, "Fold should always produce positive values") )
        .collect(),
    }
  }
}


#[derive(PartialEq, Eq, Debug)]
struct FoldInput { dots: Vec<Pos>, instructions: Vec<Instruction> }

impl FoldInput {
  fn from_file(filename: impl AsRef<Path>) -> Self {
    let mut lines = common::parse::read_lines(filename);
    let dots: Vec<Pos> = lines.by_ref()
      .take_while( |s| ! s.is_empty() )
      .map( Pos::from_str )
      .collect();

    let instructions: Vec<Instruction> = lines
      .filter( |s| ! s.is_empty() )
      .map( Instruction::from_str )
      .collect();

    FoldInput { dots, instructions }
  }
}

#[derive(Shrinkwrap)]
struct Dots(Vec<Pos>);

impl Display for Dots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let Pos(max_cols, max_rows) = self.iter().fold(Pos(0, 0), |max_pos, dot| dot.max(&max_pos) );
      let dots: HashSet<Pos> = self.iter().copied().collect();
      for j in 0..=max_rows {
        for i in 0..=max_cols {
          let c: char = if dots.contains(&Pos(i,j)) { '#'} else { ' ' };
          write!(f, "{}", c)?;
        }
        write!(f, "{}", '\n')?;
      }
      Ok(())
    }
}

#[cfg(test)]
mod test {
  use itertools::Itertools;

use super::*;

  #[test]
  fn test_parse_file() {
    let fold_input = FoldInput::from_file("../input/day13_sample.txt");
    let expected = FoldInput {
      dots: vec![
        Pos(6,10), Pos(0,14), Pos(9,10), Pos(0,3), Pos(10,4), Pos(4,11), Pos(6,0), Pos(6,12), Pos(4,1), Pos(0,13),
        Pos(10,12), Pos(3,4), Pos(3,0), Pos(8,4), Pos(1,10), Pos(2,14), Pos(8,10), Pos(9,0) ],
      instructions: vec![ Instruction::Y(7), Instruction::X(5) ],
    };
    assert_eq!(fold_input, expected);
  }

  #[test]
  fn test_fold() {
    let FoldInput { dots, .. } = FoldInput::from_file("../input/day13_sample.txt");
    let folded: Vec<Pos> = Instruction::Y(7).fold(dots).into_iter().sorted().collect();
    let expected = vec![
      Pos(0,0), Pos(0,1), Pos(0,3), Pos(1,4), Pos(2,0), Pos(3,0), Pos(3,4), Pos(4,1), Pos(4,3), Pos(6,0), Pos(6,2),
      Pos(6,4), Pos(8,4), Pos(9,0), Pos(9,4), Pos(10,2), Pos(10,4),
    ];
    assert_eq!(folded, expected);
  }

  #[test]
  fn part1() {
    let FoldInput { dots, instructions } = FoldInput::from_file("../input/day13.txt");
    let nb_folded = instructions[0].fold(dots).len();
    println!("Day 13 answer part 1 = {}", nb_folded);
    assert_eq!(nb_folded, 704);
  }

  #[test]
  fn part2() {
    let FoldInput { dots, instructions } = FoldInput::from_file("../input/day13.txt");
    let folded = instructions.into_iter().fold(dots, |dots, instruction| { instruction.fold(dots).into_iter().collect() });
    println!("{}", Dots(folded));
  }
}