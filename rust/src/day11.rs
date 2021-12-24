use std::collections::VecDeque;

use itertools::Itertools;
use ndarray::{Array2, s, iter::IndexedIterMut, ArrayViewMut2, Dim, SliceInfo, SliceInfoElem};


// trait RichArray {
//   type Index;
//   type Value;
//   fn get_adjacents_mut<'a>(&'a mut self, ij: Self::Index) -> impl Iterator<Item=(Self::Index, &'a mut Self::Value)>;
//   fn get(&self, i: Self::Index) -> Option<Self::Value>;
// }
// impl<T> RichArray for Array2<T>  {
// }

fn get_adjacents(array: &mut Array2<u8>, (i, j): (usize, usize)) -> impl Iterator<Item=(usize, usize)> {
    let (nb_rows, nb_cols) = array.dim();
    let up    = if i > 0            { i - 1 } else { 0 };
    let down  = if i < nb_rows - 1  { i + 1 } else { i };
    let left  = if j > 0            { j - 1 } else { 0 };
    let right = if j < nb_cols - 1  { j + 1 } else { j };

  (up..=down).cartesian_product(left..=right)
}


fn step(energies: &mut Array2<u8>) {
  type Index = (usize, usize);
  let mut to_visit: VecDeque<Index> = VecDeque::new();

  // 1. increase energy by 1 for all
  for (ij, e) in energies.indexed_iter_mut() {
    *e = (*e + 1) % 10;
    if *e == 0 { to_visit.push_back(ij); }
  }
  // println!("to_visit: {:?}", to_visit);
  // println!("After Increase\n{:?}", energies);

  while let Some(ij) = to_visit.pop_front() {
    let center = energies[ij];
    // println!("Visiting {:?} w/ value {}", ij, center);
    debug_assert_eq!(center, 0, "Visit only lighten octopuses");
    let adjacents = get_adjacents(energies, ij);
    for ij in adjacents {
      let e = energies.get_mut(ij).unwrap();
      if *e > 0 {
        *e = (*e + 1) % 10;
        if *e == 0 { to_visit.push_back(ij) }
      }
    }

    // println!("to_visit: {:?}", to_visit);
    // println!("visited {:?} w/ value {}\n{:?}", ij, center, energies);
  }
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

    for _ in 1..=100 { step(&mut energies); }

    assert_eq!(energies, expected100);
  }
}
