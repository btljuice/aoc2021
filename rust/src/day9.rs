use std::convert::Infallible;
use std::result::Result;
use std::str::FromStr;
use itertools::Itertools;
use ndarray::Array2;

#[derive(Debug, PartialEq)]
struct HeightMap { heights: Array2<u8> }

impl HeightMap {
  fn zeros(rows: usize, columns: usize) -> HeightMap { 
    HeightMap { heights: Array2::zeros((rows, columns)) }
  }
  fn from_shape_vec(dim: (usize, usize), numbers: Vec<u8>) -> HeightMap {
    HeightMap { heights: Array2::from_shape_vec(dim, numbers).expect("Invalid HeightMap Shape ") }
  }

  fn get(&self, i: isize, j: isize) -> Option<u8> {
      if i < 0 || j < 0 { None }
      else { self.heights.get((i as usize, j as usize)).copied() }
  }

  fn is_minima(&self, i: usize, j: usize) -> bool {
    let i = i as isize;
    let j = j as isize;
    let up    = self.get(i - 1, j    );
    let down  = self.get(i + 1, j    );
    let left  = self.get(i    , j - 1);
    let right = self.get(i    , j + 1);
    let cen   = self.get(i    , j    );

    let min_adjacents = vec![up ,down, left, right].iter().flatten().min().copied();
    
    match (cen, min_adjacents) {
      (Some(c), Some(m)) => c < m,
      (None, _) => false,
      (_, None) => true,
    }
  }

  // fn minimas(&self) -> Vec<u8> {
  //   self
  //     .heights
  //     .indexed_iter()
  //     .map(|((i, j), &h)|
  // }
}

impl FromStr for HeightMap {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
      let nb_columns = s.chars().take_while(|&c| c != '\n').count();
      let nb_rows = s.split('\n').count();

      let numbers = s .chars()
        .filter_map(|c| match c {
          '0'..='9' => c.to_digit(10).map(|d| d as u8),
          '\n' | '\t' => None,
          c if c.is_whitespace() => None,
          _ => panic!("Invalid char {}. Must be [0-9] or '\n'", c)
        }).collect_vec();

      Ok( HeightMap::from_shape_vec((nb_rows, nb_columns), numbers) )
    }
}




#[cfg(test)]
mod test {
  use ndarray::array;

  use super::HeightMap;

  const HEIGHT_MAP_STR: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

  /// **ANSME**: Couldn't figure why I couldn't define a `let height_map =` here instead. Maybe a let cannot be defined
  /// at the `mod` level
  macro_rules! HEIGHT_MAP { () => { HeightMap { heights: 
    array![[ 2,1,9,9,9,4,3,2,1,0 ],
           [ 3,9,8,7,8,9,4,9,2,1 ],
           [ 9,8,5,6,7,8,9,8,9,2 ],
           [ 8,7,6,7,8,9,6,7,8,9 ],
           [ 9,8,9,9,9,6,5,6,7,8 ]]
  } } }

  #[test] 
  fn test_from_str() {
    let height_map = HEIGHT_MAP_STR.parse::<HeightMap>().unwrap();
    assert_eq!(height_map, HEIGHT_MAP!());
  }

  #[test]
  fn test_is_minima() {
    let height_map = HEIGHT_MAP!();
    assert!(height_map.is_minima(0, 1));
    assert!(height_map.is_minima(0, 9));
    assert!(height_map.is_minima(2, 2));
    assert!(height_map.is_minima(4, 6));

    assert!(! height_map.is_minima(3, 2));
  }
}