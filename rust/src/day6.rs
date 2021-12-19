use std::str::FromStr;
use std::convert::Infallible;

use super::common;



const SPAWN_PERIOD: usize = 7;
const FIRST_SPAWN_PERIOD: usize = SPAWN_PERIOD + 2;
const SPAWN_PERIOD_MAX: usize = if SPAWN_PERIOD > FIRST_SPAWN_PERIOD { SPAWN_PERIOD } else { FIRST_SPAWN_PERIOD };
type PopSize = u64;


pub fn part1_and_2() {
   let first_line: String = common::parse::read_lines("../input/day6.txt").next().unwrap();
   let mut fishes: LanternFish = first_line.parse().unwrap();

   for _ in 1..=80 { fishes.next_day() }
   println!("day6 part1 answer = {}", fishes.total()); 

   for _ in 81..=256 { fishes.next_day() }
   println!("day6 part2 answer = {}", fishes.total()); 
}

/// population: Population by remaining days before respawn
#[derive(Clone, PartialEq, Debug)]
struct LanternFish { population: [PopSize; SPAWN_PERIOD_MAX] }
impl LanternFish {
  fn new() -> LanternFish { LanternFish { population: [0; SPAWN_PERIOD_MAX]} }

  fn total(&self) -> PopSize { self.population.iter().sum() }

  fn next_day(& mut self) {
    let nb_respawn = self.population[0];
    self.population.rotate_left(1);
    self.population[SPAWN_PERIOD - 1] += nb_respawn;
  }

  fn add_one(& mut self, i: usize) { self.population[i] += 1; }
}

impl FromStr for LanternFish {
    type Err = Infallible;

    fn from_str(numbers_str: &str) -> Result<LanternFish, Infallible> {
      let mut fishes = LanternFish::new();

      for s in numbers_str.trim().split(',') {
        let n: usize = s.parse().unwrap();
        if n > SPAWN_PERIOD_MAX { panic!("All numbers should be <= {}", SPAWN_PERIOD_MAX) }

        fishes.add_one(n);
      }

      Ok(fishes)
    }
}


#[cfg(test)]
pub(self) mod tests {
    use super::LanternFish;


  const INITIAL_STATE_STR: &str = "3,4,3,1,2";
  const INITIAL_STATE: LanternFish = LanternFish { population: [0, 1, 1, 2, 1, 0, 0, 0, 0] };

  #[test]
  fn test_from_str() {
    let fishes: LanternFish = INITIAL_STATE_STR.parse().unwrap();
    assert_eq!(fishes, INITIAL_STATE);
  }

  #[test]
  fn test_next_day_1_and_2() {
    let mut fishes = INITIAL_STATE.clone();

    // day1 
    fishes.next_day();
    assert_eq!(fishes.population, [1, 1, 2, 1, 0, 0, 0, 0, 0]);
    assert_eq!(fishes.total(), 5);

    // day2
    fishes.next_day();
    assert_eq!(fishes.population, [1, 2, 1, 0, 0, 0, 1, 0, 1]);
    assert_eq!(fishes.total(), 6);

  }

  #[test]
  fn test_next_day_18() {
    let mut fishes = INITIAL_STATE.clone();

    // day18
    for _ in 1..=18 { fishes.next_day(); }

    assert_eq!(fishes.population, [3, 5, 3, 2, 2, 1, 5, 1, 4]);
    assert_eq!(fishes.total(), 26);
  }

  #[test]
  fn test_next_day_80() {
    let mut fishes = INITIAL_STATE.clone();

    // day18
    for _ in 1..=80 { fishes.next_day(); }

    assert_eq!(fishes.total(), 5934);
  }
}