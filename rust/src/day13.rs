use lazy_regex::regex_captures;
use std::convert::AsRef;
use std::path::Path;

use crate::common;

#[derive(PartialEq, Eq, Debug)]
struct Pos(u32, u32);
impl Pos {
  fn from_str(s: impl AsRef<str>) -> Self {
    let (x, y) = s.as_ref().split_once(',').unwrap();
    Pos(x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
  }
}

#[derive(PartialEq, Eq, Debug)]
enum Instruction {
  X(u32),
  Y(u32)
}
impl Instruction {
  fn from_str(s: impl AsRef<str>) -> Self {
    let (_, axis, value) = regex_captures!(r"^fold along ([x,y])=(\d+)$", s.as_ref()).unwrap();
    let value = value.parse::<u32>().unwrap();
    match axis {
      "x" => Instruction::X(value),
      "y" => Instruction::Y(value),
      _ => panic!("Unexpected. Axis should be either x or y"),
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

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parse_file() {
    let fold_input = FoldInput::from_file("../input/day13_sample.txt");
    let expected = FoldInput {
      dots: vec![
        Pos(6,10),
        Pos(0,14),
        Pos(9,10),
        Pos(0,3),
        Pos(10,4),
        Pos(4,11),
        Pos(6,0),
        Pos(6,12),
        Pos(4,1),
        Pos(0,13),
        Pos(10,12),
        Pos(3,4),
        Pos(3,0),
        Pos(8,4),
        Pos(1,10),
        Pos(2,14),
        Pos(8,10),
        Pos(9,0),
      ],
      instructions: vec![
        Instruction::Y(7),
        Instruction::X(5),
      ],
    };
    assert_eq!(fold_input, expected);
  }
}