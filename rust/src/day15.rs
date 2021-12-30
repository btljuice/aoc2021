
use std::path::Path;
use itertools::Itertools;
use ndarray::Array2;
use shrinkwraprs::Shrinkwrap;
use crate::common;


#[derive(Shrinkwrap, PartialEq, Eq, Debug)]
struct RiskLevels(Array2<u32>);
impl RiskLevels {
  fn from_file(filename: impl AsRef<Path>) -> Self {
    let mut lines = common::parse::read_lines(filename);
    let nb_rows: usize = lines.next().unwrap().parse().unwrap();
    let nb_cols: usize = lines.next().unwrap().parse().unwrap();
    lines.next(); // Empty line

    // NOTE: Numbers are reversed here to easy indexing when computing min paths
    let mut numbers = lines.flat_map(|l|
      l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()
    ).collect_vec();
    numbers.reverse();
    
    RiskLevels(
      Array2::from_shape_vec((nb_rows, nb_cols), numbers).unwrap()
    )
  }
}

#[derive(Clone, Copy, Debug)]
enum Direction { Up, Down, Left, Right }

  // r: risk_level(i, j)
  // f(n-1, m-1) = r
  // f(n-1, m-2) = r + f(n,m)
  // f(n-2, m-1) = r + f(n,m)
  // f(n-2, m-2) = r + min{ f(n-1,m-1) ; f(n-1,m-1) }
struct MinPaths<'a> { cache: Array2<Option<(Direction, u32)>>, risks: &'a RiskLevels }
impl<'a> MinPaths<'a> {
  fn new(risks: &'a RiskLevels) -> Self {
    let (n , m) = risks.dim();
    let mut cache: Array2<Option<(Direction, u32)>> = Array2::from_elem((n, m), None);
    cache[[n-1, m-1]] = Some( (Direction::Down, risks[[n-1, m-1]] as u32) );

    MinPaths { cache, risks }
  }

}

#[cfg(test)]
mod test {
  use super::*;
  use ndarray::arr2;

  const SAMPLE: [[u32; 10]; 10] = [
    [ 1,8,5,4,4,9,1,1,3,2 ],
    [ 1,2,5,8,3,1,3,9,2,1 ],
    [ 9,3,6,1,2,4,5,2,1,3 ],
    [ 1,2,4,2,1,9,9,5,3,1 ],
    [ 7,3,1,8,2,1,9,1,3,1 ],
    [ 1,1,1,7,1,4,3,6,4,7 ],
    [ 9,6,5,1,3,9,4,9,6,3 ],
    [ 8,2,3,1,1,5,6,3,1,2 ],
    [ 2,7,6,3,7,3,1,8,3,1 ],
    [ 2,4,7,1,5,7,3,6,1,1 ],
  ];

  #[test]
  fn test_from_file() {
    let risk_levels = RiskLevels::from_file("../input/day15_sample.txt");
    let expected = RiskLevels(arr2(&SAMPLE));

    assert_eq!(risk_levels, expected);
  }
}