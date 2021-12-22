
use std::collections::VecDeque;
use std::convert::Infallible;
use std::fmt::Debug;
use std::result::Result;
use std::str::FromStr;
use itertools::Itertools;
use ndarray::Array2;
use tailcall::tailcall;
use super::common::macros::when;

#[derive(Debug, PartialEq)]
struct HeightMap { heights: Array2<u8> }

type Index = (usize, usize);


impl HeightMap {
  fn zeros(rows: usize, columns: usize) -> HeightMap { 
    HeightMap { heights: Array2::zeros((rows, columns)) }
  }
  fn from_shape_vec(dim: Index, numbers: Vec<u8>) -> HeightMap {
    HeightMap { heights: Array2::from_shape_vec(dim, numbers).expect("Invalid HeightMap Shape ") }
  }

  fn get_adjacents(&self, (i, j): Index) -> Vec<(Index, u8)> {
    let up    = when!(i > 0,          (i - 1, j));
    let down  = when!(i < usize::MAX, (i + 1, j));
    let left  = when!(j > 0,          (i, j - 1));
    let right = when!(j < usize::MAX, (i, j + 1));
    [up, down, left, right]
      .into_iter()
      .flatten()
      .filter_map(|&ij| self.get(ij).map(|v| (ij, v))
      ).collect()
  } 

  fn get(&self, ij: Index) -> Option<u8> { self.heights.get(ij).copied() }

  fn is_minima(&self, ij: Index) -> bool {
    let center = self.get(ij);
    let min_adjacents = self.get_adjacents(ij).into_iter().map(|(_, v)| v).min();
    
    match (center, min_adjacents) {
      (Some(c), Some(m)) => c < m,
      (None, _) => false,
      (_, None) => true,
    }
  }

  fn minimas(&self) -> Vec<u8> {
    self
      .heights
      .indexed_iter()
      .filter_map( |(ij, &h)| when!(self.is_minima(ij), h + 1) )
      .collect()
  }

  fn largest_basin_sizes(&self, nth_largest: usize) -> Vec<usize> {
    self
      .heights
      .indexed_iter()
      .map( |(ij,_)| self.basin(ij).1 )
      .fold(Vec::<usize>::new(), |mut largest, sz| {
        largest.push(sz);
        largest.sort_by(|a, b| b.cmp(a));
        if largest.len() > nth_largest { largest.pop(); }
        largest
      })
  }

  /// Tailcall exploration. For this one, since both `visited and `to_visit` are mutable structures, I tend to think
  /// a while loop would have been better, but wanted to explore tailcalls in Rust.
  /// 
  /// **OPTME**: accrue the basin size, since this is the goal and we don't really care about visited.
  #[tailcall]
  fn basin_impl(hmap: &HeightMap, mut visited: Array2<bool>, mut to_visit: VecDeque<Index>, nb_visited: usize) -> (Array2<bool>, usize) {
    let already_visited = |ij: Index| -> bool { 
      visited.get(ij).copied().unwrap_or(true) // Out-of-bound is considered already visited
    };

    match to_visit.pop_front() {
      None => (visited, nb_visited),
      Some(ij) if already_visited(ij) => basin_impl(hmap, visited, to_visit, nb_visited),
      Some(ij) => {
        let center = hmap.get(ij).expect("Unexpected. All indexes at this point should be valid and unvisited");
        let mut adjacents_to_visit: VecDeque<Index> = hmap
          .get_adjacents(ij)
          .into_iter()
          .filter_map(|(ij, v)| when!(center < v && v < 9, ij) ) // SMALL-OPTME: Call already_visited to make less tailcalls.
          .collect();
        
        to_visit.append(&mut adjacents_to_visit);
        visited[ij] = true;

        basin_impl(hmap, visited, to_visit, nb_visited + 1)
      }
    }
  }

  fn basin(&self, ij: Index) -> (Array2<bool>, usize) {
    let visited = Array2::<bool>::default(self.heights.dim());
    let mut to_visit: VecDeque<Index> = VecDeque::new();
    to_visit.push_back(ij);
    
    HeightMap::basin_impl(&self, visited, to_visit, 0)
  }

  fn basin_size(basin: &Array2<bool>) -> usize { basin.iter().copied().map_into::<usize>().sum() }
}

impl FromStr for HeightMap {
    type Err = Infallible; // TODO: Change to !
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
  use itertools::Itertools;
  use ndarray::array;
  use ndarray::Array2;
  use std::fs;

  use super::HeightMap;
use super::Index;

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
    assert!( height_map.is_minima((0, 1)) );
    assert!( height_map.is_minima((0, 9)) );
    assert!( height_map.is_minima((2, 2)) );
    assert!( height_map.is_minima((4, 6)) );

    assert!( !height_map.is_minima((3, 2)) );
  }

  #[test]
  fn test_minimas() {
    let height_map = HEIGHT_MAP!();
    let mut mins = height_map.minimas();
    mins.sort();
    assert_eq!(mins, vec![1, 2, 6, 6])
  }

  #[test]
  fn test_part1() {
    let content = fs::read_to_string("../input/day9.txt").unwrap();
    let height_map: HeightMap = content.parse::<HeightMap>().unwrap(); // TODO: replace by into_ok() when ! gets stabilized
    let sum: u32 = height_map.minimas().into_iter().map_into::<u32>().sum();
    println!("day9 part 1 answer = {}", sum);
    assert_eq!(sum, 494);
  }

  #[test]
  fn test_basins() {
    test_basin( (0, 1), vec![(0, 0), (0, 1), (1, 0)] );

    test_basin( (0, 9), vec![
      (0, 5), (0, 6), (0, 7), (0, 8), (0, 9),
      (1, 6), (1, 8), (1, 9),
      (2, 9)
    ]);  

    test_basin( (2, 2), vec![
      (1, 2), (1, 3), (1, 4),
      (2, 1), (2, 2), (2, 3), (2, 4), (2, 5),
      (3, 0), (3, 1), (3, 2), (3, 3), (3, 4), 
      (4, 1)
    ]);  

    test_basin( (4, 6), vec![
      (2, 7),
      (3, 6), (3, 7), (3, 8),
      (4, 5), (4, 6), (4, 7), (4, 8), (4, 9), 
    ]);  
  }

  #[test]
  fn test_largest_basin_sizes() {
    let height_map = HEIGHT_MAP!();
    let largest = height_map.largest_basin_sizes(3);
    assert_eq!(largest, vec![14, 9, 9]);
  }

  #[test]
  fn test_part2() {
    let content = fs::read_to_string("../input/day9.txt").unwrap();
    let height_map: HeightMap = content.parse::<HeightMap>().unwrap(); // TODO: replace by into_ok() when ! gets stabilized
    let largest = height_map.largest_basin_sizes(3);
    let answer = largest.into_iter().product::<usize>();
    println!("day9 part 2 answer = {}", answer);
    assert_eq!(answer, 1048128);
  }

  fn test_basin(center: Index, expected: Vec<Index>) {
    let height_map = HEIGHT_MAP!();
    let (basin, _) = height_map.basin(center);

    let mut expected_basin = Array2::<bool>::default((5, 10));
    for ij in expected { expected_basin[ij] = true; }

    assert_eq!(basin, expected_basin);
  }


}