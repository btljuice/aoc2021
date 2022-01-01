
use std::{path::Path, convert::identity, cmp::Ordering, ops::Add};
use itertools::Itertools;
use ndarray::Array2;
use shrinkwraprs::Shrinkwrap;
use crate::common::{self, macros::when};

type Pos = (usize, usize);

#[derive(Shrinkwrap, PartialEq, Eq, Debug)]
struct RiskLevels(Array2<u32>);
impl RiskLevels {
  fn from_file(filename: impl AsRef<Path>) -> Self {
    let mut lines = common::parse::read_lines(filename);
    let nb_rows: usize = lines.next().unwrap().parse().unwrap();
    let nb_cols: usize = lines.next().unwrap().parse().unwrap();
    lines.next(); // Empty line

    let numbers = lines.flat_map(|l|
      l.chars().map(|c| c.to_digit(10).unwrap()).collect_vec()
    ).collect_vec();
    
    RiskLevels(
      Array2::from_shape_vec((nb_rows, nb_cols), numbers).unwrap()
    )
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MinPath { cost: u32,  dirs: Vec<Pos> }
impl MinPath {
  fn some(cost: u32, dirs: Vec<Pos>) -> Option<Self> { Some( MinPath { cost, dirs } ) }
}

impl PartialOrd for MinPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.cost.partial_cmp(&other.cost) }
}

impl Ord for MinPath {
    fn cmp(&self, other: &Self) -> Ordering { self.cost.cmp(&other.cost) }
}

impl Add<(u32, Pos)> for MinPath {
  type Output = MinPath;

  fn add(self, (cost, dir): (u32, Pos)) -> Self::Output {
    let mut dirs = self.dirs.clone();
    dirs.push(dir);
    MinPath { cost: self.cost + cost, dirs }
  }
}

  // Min paths can start to be computed the following way
  // r: risk_level(i, j)
  // f(0,0) = 0
  // f(0,1) = r
  // f(1,0) = r
  // f(1,1) = r + min{ f(0,1) ; f(1,0) }
  // **TODO**: Use cache macro instead of manual `cache` computation
struct MinPaths<'a> { cache: Array2<Option<MinPath>>, risks: &'a RiskLevels }
impl<'a> MinPaths<'a> {
  fn new(risks: &'a RiskLevels) -> Self {
    let mut cache: Array2<Option<MinPath>> = Array2::<Option<MinPath>>::from_elem(risks.dim(), None);
    let r_01 = risks[[0,1]];
    let r_10 = risks[[1,0]];
    let r_11 = risks[[1,1]];
    cache[[0,0]] = MinPath::some(   0, vec![(0,0)      ]);
    cache[[0,1]] = MinPath::some(r_01, vec![(0,0), (0,1)]);
    cache[[1,0]] = MinPath::some(r_10, vec![(0,0), (1,0)]);

    cache[[1,1]] = if r_01 < r_10 { MinPath::some(r_11, vec![(0,0), (0,1), (1,1)]) }
                   else           { MinPath::some(r_11, vec![(0,0), (1,0), (1,1)]) };

    MinPaths { cache, risks }
  }

  fn cache_get(&'a self, ij: Pos) -> Option<&'a MinPath> { self.cache.get(ij).unwrap_or(&None).as_ref() }

  fn nrows(&self) -> usize { self.risks.nrows() }
  fn ncols(&self) -> usize { self.risks.ncols() }


  fn up(   &self, (i,j): Pos) -> Option<Pos> { when!( j > 0           , (i  ,j-1) ) }
  fn down( &self, (i,j): Pos) -> Option<Pos> { when!( j < self.nrows(), (i  ,j+1) ) }
  fn left( &self, (i,j): Pos) -> Option<Pos> { when!( i > 0           , (i-1,j  ) ) }
  fn right(&self, (i,j): Pos) -> Option<Pos> { when!( i < self.ncols(), (i+1,j  ) ) }

  fn cache_up(   &'a self, ij: Pos) -> Option<&'a MinPath> { self.up(   ij).and_then(|p| self.cache_get(p)) }
  fn cache_right(&'a self, ij: Pos) -> Option<&'a MinPath> { self.right(ij).and_then(|p| self.cache_get(p)) }
  fn cache_down( &'a self, ij: Pos) -> Option<&'a MinPath> { self.down( ij).and_then(|p| self.cache_get(p)) }
  fn cache_left( &'a self, ij: Pos) -> Option<&'a MinPath> { self.left( ij).and_then(|p| self.cache_get(p)) }

  fn cache_min_neighbor(&'a self, ij: Pos) -> Option<&'a MinPath> {
    [ self.cache_up(ij), 
      self.cache_down(ij), 
      self.cache_right(ij), 
      self.cache_left(ij) 
    ].into_iter().flatten().min()
  }

  fn min_path(&'a mut self, ij: Pos) -> &'a MinPath {
    // Return cached value if it exists
    if let Some(min_path) = self.cache_get(ij) { return min_path; }

    // Look for minimal paths through cached neighbors
    let min_cache_neighbor = self.cache_min_neighbor(ij).expect("min_path should be called only next to at least one neighbor that is already computed");

    let up    =    self.up(ij).and_then(|p| self.min_path_impl(p, vec![ij], min_cache_neighbor.cost)); 
    let down  =  self.down(ij).and_then(|p| self.min_path_impl(p, vec![ij], min_cache_neighbor.cost)); 
    let right = self.right(ij).and_then(|p| self.min_path_impl(p, vec![ij], min_cache_neighbor.cost)); 
    let left  =  self.left(ij).and_then(|p| self.min_path_impl(p, vec![ij], min_cache_neighbor.cost)); 

    let min = [ up, down, right, left ].into_iter().flatten().min().unwrap();

    let cache_ref: &'a mut Option<MinPath> = self.cache.get_mut(ij).unwrap();
    *cache_ref = Some(min + (self.risks[ij], ij));

    self.cache_get(ij).unwrap()
  }


  fn min_path_impl(&'a self, ij: Pos, visited: Vec<Pos>, max_length: u32) -> Option<MinPath> {
    todo!()
  }

}

#[cfg(test)]
mod test {
  use super::*;
  use ndarray::arr2;

  const SAMPLE: [[u32; 10]; 10] = [
    [ 1,1,6,3,7,5,1,7,4,2 ],
    [ 1,3,8,1,3,7,3,6,7,2 ],
    [ 2,1,3,6,5,1,1,3,2,8 ],
    [ 3,6,9,4,9,3,1,5,6,9 ],
    [ 7,4,6,3,4,1,7,1,1,1 ],
    [ 1,3,1,9,1,2,8,1,3,7 ],
    [ 1,3,5,9,9,1,2,4,2,1 ],
    [ 3,1,2,5,4,2,1,6,3,9 ],
    [ 1,2,9,3,1,3,8,5,2,1 ],
    [ 2,3,1,1,9,4,4,5,8,1 ],
  ];

  #[test]
  fn test_from_file() {
    let risk_levels = RiskLevels::from_file("../input/day15_sample.txt");
    let expected = RiskLevels(arr2(&SAMPLE));

    assert_eq!(risk_levels, expected);
  }
}