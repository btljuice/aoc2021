use std::convert::identity;
use std::collections::HashMap;
use std::path::Path;
use itertools::MinMaxResult;
use itertools::Itertools;
use lazy_regex::regex_captures;
use shrinkwraprs::Shrinkwrap;

use crate::common;
use crate::common::collections::freq_count;

type Pair = (char, char);

struct Inputs {
  polymer: String,
  insertion_rules: Rules,
}
impl Inputs {
  fn from_file(filename: impl AsRef<Path>) -> Self {
    fn first_char(s: &str) -> char { s.chars().next().unwrap() }

    let mut lines = common::parse::read_lines(filename);
    let polymer = lines.next().unwrap();

    let rules: HashMap<Pair, char> = lines
      .filter(|l| ! l.is_empty() )
      .fold(HashMap::new(), |mut acc, l| {
        let (_, a, b, c) = regex_captures!(r"^([A-Z])([A-Z]) -> ([A-Z])$", l.as_str()).unwrap();
        acc.insert((first_char(a), first_char(b)), first_char(c));
        acc
      });

    Inputs { polymer, insertion_rules: Rules(rules) }
  }
}

#[derive(Shrinkwrap)]
struct Rules(HashMap<Pair, char>);
impl Rules {
  fn insert_elements(&self, polymer: String) -> String {
    polymer.chars().fold(String::new(), |mut acc, b| {
      if let Some(a) = acc.chars().last() {
        if let Some(&c) = self.get(&(a, b)) {
          acc.push(c);
        }
      }
      acc.push(b);
      acc
    })
  }

  fn expand_polymer(&self, polymer: String, nb_steps: usize) -> HashMap<char, u64> {
    let mut counts = freq_count(polymer.chars());
    let mut pairs: HashMap<Pair, u64> = freq_count(polymer.chars().tuple_windows());

    // Equivalent to a fold
    // (1..=nb_steps).fold(pairs, |pairs, _| self.pair_insertion(pairs, &mut counts));
    for _ in 1..=nb_steps { 
      pairs = self.pair_insertion(pairs, &mut counts); 
    }

    counts
  }

  fn pair_insertion(&self, pairs: HashMap<Pair, u64>, counts: &mut HashMap<char, u64>) -> HashMap<Pair, u64> {
    let mut new_pairs: HashMap<Pair, u64> = HashMap::new();
    for ((a, c), nb_pairs) in pairs {
      let &b = self.get(&(a,c)).expect("All combination pairs should exist in the provided rules");
      *counts.entry(b).or_default() += nb_pairs;
      *new_pairs.entry((a, b)).or_default() += nb_pairs;
      *new_pairs.entry((b, c)).or_default() += nb_pairs;
    }
    new_pairs
  }
}

fn minmax_str(s: impl AsRef<str>) -> MinMaxResult<(char, u64)> {
  let freq_counts = common::collections::freq_count(s.as_ref().chars());
  minmax(&freq_counts)
}

fn minmax(counts: &HashMap<char, u64>) -> MinMaxResult<(char, u64)> {
  counts.iter().map(|(&k, &v)|(k, v)).minmax_by( |(_, n_a), (_, n_b)| n_a.cmp(n_b) )
}

#[cfg(test)]
mod test {
  use itertools::Itertools;

use super::*;

  #[test]
  fn test_from_file() {
    let Inputs { polymer, insertion_rules } = Inputs::from_file("../input/day14_sample.txt");
    let Rules(insertion_rules) = insertion_rules; 
    let insertion_rules: Vec<_> = insertion_rules.into_iter().sorted().collect();
    let expected_rules = vec![
      (('B', 'B'), 'N'),
      (('B', 'C'), 'B'),
      (('B', 'H'), 'H'),
      (('B', 'N'), 'B'),
      (('C', 'B'), 'H'),
      (('C', 'C'), 'N'),
      (('C', 'H'), 'B'),
      (('C', 'N'), 'C'),
      (('H', 'B'), 'C'),
      (('H', 'C'), 'B'),
      (('H', 'H'), 'N'),
      (('H', 'N'), 'C'),
      (('N', 'B'), 'B'),
      (('N', 'C'), 'B'),
      (('N', 'H'), 'C'),
      (('N', 'N'), 'C'),
    ];

    assert_eq!(polymer, "NNCB");
    assert_eq!(insertion_rules, expected_rules);
  }

  #[test]
  fn test_insert_elements() {
    let Inputs { polymer, insertion_rules } = Inputs::from_file("../input/day14_sample.txt");

    let polymer = insertion_rules.insert_elements(polymer);
    assert_eq!(polymer, "NCNBCHB");

    let polymer = insertion_rules.insert_elements(polymer);
    assert_eq!(polymer, "NBCCNBBBCBHCB");

    let polymer = insertion_rules.insert_elements(polymer);
    assert_eq!(polymer, "NBBBCNCCNBBNBNBBCHBHHBCHB");

    let polymer = insertion_rules.insert_elements(polymer);
    assert_eq!(polymer, "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB");
  }

  #[test]
  fn test_expand_polymer() {
    let Inputs { polymer, insertion_rules } = Inputs::from_file("../input/day14_sample.txt");
    let count = insertion_rules.expand_polymer(polymer, 4);
    let expected_counts = common::collections::freq_count("NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB".chars());

    assert_eq!(count, expected_counts);
    

  }

  #[test]
  fn test_minmax() {
    let Inputs { mut polymer, insertion_rules } = Inputs::from_file("../input/day14_sample.txt");

    for _ in 1..=10 { polymer = insertion_rules.insert_elements(polymer); }
    
    assert_eq!(polymer.len(), 3073);
    assert_eq!(minmax_str(polymer), MinMaxResult::MinMax(('H', 161), ('B', 1749)));
  }

  #[test]
  fn test_minmax_with_expand_polymer() {
    let Inputs { polymer, insertion_rules } = Inputs::from_file("../input/day14_sample.txt");

    let count = insertion_rules.expand_polymer(polymer, 10);

    assert_eq!(minmax(&count), MinMaxResult::MinMax(('H', 161), ('B', 1749)));

  }

  #[test]
  fn part1() {
    let Inputs { mut polymer, insertion_rules } = Inputs::from_file("../input/day14.txt");

    for _ in 1..=10 { polymer = insertion_rules.insert_elements(polymer); }

    if let MinMaxResult::MinMax((_, min), (_, max)) = minmax_str(polymer) {
      let answer = max - min;
      println!("day14 part1 answer = {}", max - min);
      assert_eq!(answer, 2010);
    } else { panic!("Unexpected"); }
  }

  #[test]
  fn part2() {
    let Inputs { mut polymer, insertion_rules } = Inputs::from_file("../input/day14.txt");
    let count = insertion_rules.expand_polymer(polymer, 40);

    if let MinMaxResult::MinMax((_, min), (_, max)) = minmax(&count) {
      let answer = max - min;
      println!("day14 part2 answer = {}", max - min);
      assert_eq!(answer, 2437698971143);
    } else { panic!("Unexpected"); }

  }

}