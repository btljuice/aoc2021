use std::collections::HashMap;
use std::path::Path;
use lazy_regex::regex_captures;
use shrinkwraprs::Shrinkwrap;

use crate::common;



struct Inputs {
  polymer: String,
  insertion_rules: HashMap<(char, char), char>,
}
impl Inputs {
  fn from_file(filename: impl AsRef<Path>) -> Self {
    fn first_char(s: &str) -> char { s.chars().next().unwrap() }

    let mut lines = common::parse::read_lines(filename);
    let polymer = lines.next().unwrap();

    let insertion_rules: HashMap<(char, char), char> = lines
      .filter(|l| ! l.is_empty() )
      .fold(HashMap::new(), |mut acc, l| {
        let (_, a, b, c) = regex_captures!(r"^([A-Z])([A-Z]) -> ([A-Z])$", l.as_str()).unwrap();
        acc.insert((first_char(a), first_char(b)), first_char(c));
        acc
      });

    Inputs { polymer, insertion_rules }
  }
}

#[derive(Shrinkwrap)]
struct Rules(HashMap<(char, char), char>);
impl Rules {
  fn pair_insertion(&self, polymer: String) -> String {
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
}

#[cfg(test)]
mod test {
  use itertools::Itertools;

use super::*;

  #[test]
  fn test_from_file() {
    let Inputs { polymer, insertion_rules } = Inputs::from_file("../input/day14_sample.txt");
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
  fn test_pair_insertion() {
    let Inputs { polymer, insertion_rules } = Inputs::from_file("../input/day14_sample.txt");
    let insertion_rules = Rules(insertion_rules);

    let polymer = insertion_rules.pair_insertion(polymer);
    assert_eq!(polymer, "NCNBCHB");

    let polymer = insertion_rules.pair_insertion(polymer);
    assert_eq!(polymer, "NBCCNBBBCBHCB");

    let polymer = insertion_rules.pair_insertion(polymer);
    assert_eq!(polymer, "NBBBCNCCNBBNBNBBCHBHHBCHB");

    let polymer = insertion_rules.pair_insertion(polymer);
    assert_eq!(polymer, "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB");
  }


}