use std::collections::VecDeque;

use itertools::Itertools;
use ndarray::{Array2, s, iter::IndexedIterMut, ArrayViewMut2, Dim, SliceInfo, SliceInfoElem};

trait Adjacents {
  type Index;
  /// **ANSME**: 
  ///   - How return a generic Iterator (e.g. impl Iterator<Item=Index>)
  ///   - How return a generic Iterator w/ references or mutable references (e.g. impl Iterator<Item=&mut T>)
  fn get_adjacents(&self, i: Self::Index) -> Vec<Self::Index>;
}

impl Adjacents for Array2<u8> {
    type Index = (usize, usize);

    fn get_adjacents(&self, (i, j): Self::Index) -> Vec<Self::Index> {
      let (nb_rows, nb_cols) = self.dim();
      let up    = if i > 0            { i - 1 } else { 0 };
      let down  = if i < nb_rows - 1  { i + 1 } else { i };
      let left  = if j > 0            { j - 1 } else { 0 };
      let right = if j < nb_cols - 1  { j + 1 } else { j };

      (up..=down).cartesian_product(left..=right).collect()
    }
}

/// **returns**: number of lighten octopuses
fn step(energies: &mut Array2<u8>) -> usize {
  type Index = (usize, usize);
  let mut to_visit: VecDeque<Index> = VecDeque::new();
  let mut nb_lightened: usize = 0;

  let mut increase_energy = |ij: (usize, usize), e: &mut u8| {
    *e = (*e + 1) % 10;
    if *e == 0 { 
      nb_lightened += 1;
      to_visit.push_back(ij); 
    }
  };

  // 1. increase energy by 1 for all
  for (ij, e) in energies.indexed_iter_mut() { increase_energy(ij, e); }
  // println!("to_visit: {:?}", to_visit);
  // println!("After Increase\n{:?}", energies);

  while let Some(ij) = to_visit.pop_front() {
    let center = energies[ij];
    // println!("Visiting {:?} w/ value {}", ij, center);
    debug_assert_eq!(center, 0, "Visit only lighten octopuses");
    for ij in energies.get_adjacents(ij) {
      let e = energies.get_mut(ij).unwrap();
      if *e > 0 { increase_energy(ij, e) }
    }
  }

  // println!("to_visit: {:?}", to_visit);
  // println!("visited {:?} w/ value {}\n{:?}", ij, center, energies);

  nb_lightened
}


#[cfg(test)]
mod test {
  use ndarray::Array2;
  use ndarray::array;
  use crate::day11::*;

  #[test]
  fn test_step1() {
    let initial: Array2<u8> = array![ 
      [ 1, 1, 1, 1, 1 ],
      [ 1, 9, 9, 9, 1 ],
      [ 1, 9, 1, 9, 1 ],
      [ 1, 9, 9, 9, 1 ],
      [ 1, 1, 1, 1, 1 ],
    ];

    let expected1: Array2<u8> = array![
      [ 3, 4, 5, 4, 3 ],
      [ 4, 0, 0, 0, 4 ],
      [ 5, 0, 0, 0, 5 ],
      [ 4, 0, 0, 0, 4 ],
      [ 3, 4, 5, 4, 3 ],
    ];

    let expected2: Array2<u8> = array![
      [ 4, 5, 6, 5, 4 ],
      [ 5, 1, 1, 1, 5 ],
      [ 6, 1, 1, 1, 6 ],
      [ 5, 1, 1, 1, 5 ],
      [ 4, 5, 6, 5, 4 ],
    ];

    let mut energies = initial.clone();
    step(&mut energies);
    assert_eq!(energies, expected1);
    step(&mut energies);
    assert_eq!(energies, expected2);
  }

  #[test]
  fn test_step100() {
    let mut energies: Array2<u8> = array![ 
      [ 5,4,8,3,1,4,3,2,2,3 ],
      [ 2,7,4,5,8,5,4,7,1,1 ],
      [ 5,2,6,4,5,5,6,1,7,3 ],
      [ 6,1,4,1,3,3,6,1,4,6 ],
      [ 6,3,5,7,3,8,5,4,7,8 ],
      [ 4,1,6,7,5,2,4,6,4,5 ],
      [ 2,1,7,6,8,4,1,7,2,1 ],
      [ 6,8,8,2,8,8,1,1,3,4 ],
      [ 4,8,4,6,8,4,8,5,5,4 ],
      [ 5,2,8,3,7,5,1,5,2,6 ],
    ]; 

    let expected100: Array2<u8> = array![
      [ 0,3,9,7,6,6,6,8,6,6 ],
      [ 0,7,4,9,7,6,6,9,1,8 ],
      [ 0,0,5,3,9,7,6,9,3,3 ],
      [ 0,0,0,4,2,9,7,8,2,2 ],
      [ 0,0,0,4,2,2,9,8,9,2 ],
      [ 0,0,5,3,2,2,2,8,7,7 ],
      [ 0,5,3,2,2,2,2,9,6,6 ],
      [ 9,3,2,2,2,2,8,9,6,6 ],
      [ 7,9,2,2,2,8,6,8,6,6 ],
      [ 6,7,8,9,9,9,8,7,6,6 ],
    ];

    let nb_lightened: usize = (1..=100).into_iter().map(|_| step(&mut energies)).sum();

    assert_eq!(energies, expected100);
    assert_eq!(nb_lightened, 1656);

    // Test all flash
    let mut nb_steps: usize = 100;
    loop {
      nb_steps += 1;
      step(&mut energies);
      if energies.iter().all(|&e| e == 0) { break; }
    }

    assert_eq!(nb_steps, 195);
  }

  #[test]
  fn part1_and_2() {

    let mut energies: Array2<u8> = array![ 
      [ 5,4,2,1,4,5,1,7,4,1 ],
      [ 3,8,7,7,3,2,1,5,6,8 ],
      [ 7,5,8,3,2,7,3,8,6,4 ],
      [ 3,4,5,1,7,1,7,7,7,8 ],
      [ 2,6,5,1,6,1,5,1,5,6 ],
      [ 6,3,7,7,1,6,7,5,2,6 ],
      [ 5,1,8,2,8,5,2,8,3,1 ],
      [ 4,7,6,6,8,5,6,6,7,6 ],
      [ 3,4,3,7,1,8,7,5,8,3 ],
      [ 3,6,3,3,3,7,1,5,8,6 ],
    ]; 
    let nb_lightened: usize = (1..=100).into_iter().map(|_| step(&mut energies)).sum();
    println!("day11 part 1 answer= {}", nb_lightened);
    assert_eq!(nb_lightened, 1673);

    // Test all flash
    let mut nb_steps: usize = 100;
    loop {
      nb_steps += 1;
      step(&mut energies);
      if energies.iter().all(|&e| e == 0) { break; }
    }

    println!("day11 part 2 answer= {}", nb_steps);
  }
}
