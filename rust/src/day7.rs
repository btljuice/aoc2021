use rdxsort::*;
use conv::*;
use itertools::Itertools;

use super::common::delta;
use super::common::sum_n;

type PosType = u16;

pub(self) struct Crabs { positions: Vec<PosType> }
impl Crabs {
  pub fn displace<F>(&self, cost_fn: F) -> u32 where F: Fn(PosType) -> u32 { self.positions.iter().map(|&p| cost_fn(p)).sum() }
  pub fn sorted(&mut self) { self.positions.rdxsort(); }
}

/// Strategy here is to compute the displacement at the median.
/// The cost displacement function is of the form |x - c|
/// When aggregating |x - c| functions, in can be shown that the minimum is at the median region
/// of all aggregate functions.
pub(self) fn min_displace_part1(crabs: &mut Crabs) -> u32 {
    if crabs.positions.is_empty() { return 0; }
    crabs.sorted();

    let n = crabs.positions.len();
    // For an even number of crabs (absolute functions) any point between the 2 medians constitute
    // the minimal plateau. Thus we just pick the median at the odd index.
    let median = crabs.positions[n / 2];

    crabs.displace(|p| delta(median, p) as u32)
}

///
/// cost function = ∑ i for i = 0..|x-p|
///       c(x, p) = (|x-p|)(|x-p| + 1) / 2
///               = ( x^2 -2px +p^2 + |x-p| )/ 2
///
/// derivative of cost function
/// c(x,p)' = x - p - 1/2   for [...p[
///         = x - p + 1/2   for [p..[
///
/// sum of aggregated cost functions
/// ∑ c' = nx - ∑ p + [(n-i)*(-1/2) + i*1/2]
///      = nx -  P  + [-n/2 + i]
///      = nx -  (P+n/2) + i
///      = nx - C + i
/// 1st term nx increases with x
/// 2nd term P-n/2 is constant
/// 3rd i term is step-wise increasing with x
/// Thus derivative is ever increasing
///
/// Can be shown through the 2nd derivative as well.
/// ∑ c'' = n  // Positive curvature
///
/// There's only one minimum, we just need to scan through positions to find out where the derivative
/// goes from negative to positive.
///
/// Examples in graph: https://www.desmos.com/calculator/7i48ybrrz5
/// The aggregated cost
pub(self) fn min_displace_part2(crabs: &mut Crabs) -> u32 {
  fn cost(a: PosType, b: PosType) -> u32 { sum_n(delta(a, b) as u32) }
  fn total_cost(crabs: &Crabs, a: PosType) -> u32 { crabs.displace(|b| cost(a, b)) }

  if crabs.positions.is_empty() { return 0; }
  crabs.sorted();

  let n: f64 = crabs.positions.len() as f64;
  let P: f64 = crabs.positions.iter().map(|&p| f64::from(p)).sum();
  let C: f64 = P + 0.5*n;

  let d_cost = |x: PosType, i: usize| -> f64 { n * f64::from(x) + f64::value_from(i).unwrap() - C };

  for (i, (&x0, &x1)) in crabs.positions.iter().tuple_windows::<(&PosType,&PosType)>().enumerate() {
    let d1 = d_cost(x1, i+1); // derivative on the left of x1
    if d1 < 0.0 { continue; } // function is still decreasing between [x0, x1[

    let d0 = d_cost(x0, i+1); // derivative on the right of x0
    if d0 >= 0.0 {
      return total_cost(crabs, x0);
    } // both derivative are increasing. Return x0
    else {
      let x_min: f64 = (C - f64::value_from(i).unwrap()) / n;
      let x_min0: PosType = x_min.ceil() as PosType;
      let x_min1: PosType = x_min.floor() as PosType;
      let c_min0: u32 = total_cost(crabs, x_min0);
      let c_min1: u32 = total_cost(crabs, x_min1);
      return c_min0.min(c_min1);
    }
  }

  let &last = crabs.positions.last().unwrap();
  total_cost(crabs, last)
}

#[cfg(test)]
mod test {
  use crate::common;
  use crate::common::delta;
  use crate::day7::{min_displace_part1, min_displace_part2};
  use super::PosType;
  use super::Crabs;

  const POSITIONS: [PosType; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

  #[test]
  fn test_displace() {
    let crabs = Crabs { positions: POSITIONS.to_vec() };

    fn cost(a: PosType, b: PosType) -> u32 { delta(a, b) as u32 }

    assert_eq!(crabs.displace(|p| cost(1, p)), 41);
    assert_eq!(crabs.displace(|p| cost(2, p)), 37);
    assert_eq!(crabs.displace(|p| cost(3, p)), 39);
    assert_eq!(crabs.displace(|p| cost(10, p)), 71);
  }

  #[test]
  fn test_min_displace_part1() {
    let mut crabs = Crabs { positions: POSITIONS.to_vec() };
    assert_eq!(min_displace_part1(&mut crabs), 37);
  }

  #[test]
  fn part1() {
    let positions = common::read_comma_separated("../input/day7.txt");
    let mut crabs = Crabs { positions };
    let sln = min_displace_part1(&mut crabs);
    println!("Part1 solution = {}", sln);
  }

  #[test]
  fn test_min_displace_part2() {
    let mut crabs = Crabs { positions: POSITIONS.to_vec() };
    assert_eq!(min_displace_part2(&mut crabs), 168);
  }

  #[test]
  fn part2() {
    let positions = common::read_comma_separated("../input/day7.txt");
    let mut crabs = Crabs { positions };
    let sln = min_displace_part2(&mut crabs);
    println!("Part2 solution = {}", sln);
  }
}
