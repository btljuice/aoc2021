use super::common;

type PosType = u16;

struct Crabs { positions: Vec<PosType> }

pub(self) mod part1 {
  use super::PosType;
  use super::common::delta;
  use rdxsort::*;


  impl super::Crabs {
    pub fn displace(&self, to: PosType) -> u32 {
      self.positions.iter().map(|&n| delta(n, to)  as u32).sum()
    }

    /// Strategy here is to compute the displacement at the median.
    /// The cost displacement function is of the form |x - c|
    /// When aggregating |x - c| functions, in can be shown that the minimum is at the median region
    /// of all aggregate functions.
    pub fn min_displace(&mut self) -> u32 {
      if self.positions.is_empty() { return 0; }
      self.positions.rdxsort();

      let n = self.positions.len();
      // For an even number of crabs (absolute functions) any point between the 2 medians constitute
      // the minimal plateau. Thus we just pick the median at the odd index.
      let median = self.positions[n / 2];
      self.displace(median)
    }
  }
}

#[cfg(test)]
mod test_part1 {
  use super::PosType;
  use super::Crabs;
  use super::common;
  use super::part1::*;

  const POSITIONS: [PosType; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

  #[test]
  fn test_displace() {
    let crabs = Crabs { positions: POSITIONS.to_vec() };

    assert_eq!(crabs.displace(1), 41);
    assert_eq!(crabs.displace(2), 37);
    assert_eq!(crabs.displace(3), 39);
    assert_eq!(crabs.displace(10), 71);
  }

  #[test]
  fn test_min_displace() {
    let mut crabs = Crabs { positions: POSITIONS.to_vec() };
    assert_eq!(crabs.min_displace(), 37);
  }

  #[test]
  fn part1() {
    let positions = common::read_comma_separated("../input/day7.txt");
    let mut crabs = Crabs { positions };
    let sln = crabs.min_displace();
    println!("Part1 solution = {}", sln);
  }
}
