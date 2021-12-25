use std::collections::HashMap;
use std::convert::Infallible;
use std::str::FromStr;
use std::fmt::{Debug, Display};
use std::hash::Hash;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Node {
  Start,
  BigCave(String),
  SmallCave(String),
  End,
}

// Traits Aliases
trait KeyTraits: Sized + Clone + Hash + Eq + Ord {}
impl<T> KeyTraits for T where T: Sized + Clone + Hash + Eq + Ord {}

impl FromStr for Node {
  type Err = Infallible; // TODO: Change to ! never type when it becomes a stable feature

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "start" => Ok(Node::Start),
      "end" => Ok(Node::End),
      _ if s.chars().all(|c| c.is_lowercase()) => Ok(Node::SmallCave(s.to_string())),
      _ => Ok(Node::BigCave(s.to_string())),
    }
  }
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        Node::Start => write!(f, "start"),
        Node::End => write!(f, "end"),
        Node::SmallCave(id) => write!(f, "{}", id),
        Node::BigCave(id) => write!(f, "{}", id),
      }
    }
}

/// **todo**: Look for a well established graph library 
#[derive(Debug)]
struct BiGraph<V> { edges: HashMap<V, Vec<V>> }

impl<V: Ord + Clone> BiGraph<V> {
  fn edges_vec(&self) -> Vec<(V, V)> {
    let mut acc: Vec<(V, V)> = Vec::new();
    for (k, vs) in self.edges.iter() { 
      for v in vs {
        if k < v { acc.push((k.clone(), v.clone())); }
      }
    }

    acc.sort();
    acc
  }
}

impl<V> FromIterator<(V, V)> for BiGraph<V> where V: KeyTraits {
  fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = (V, V)> {
    let edges = iter.into_iter().flat_map(|(a, b)| [(a.clone(),b.clone()), (b, a)]); // Graph is bidirectional
    let map = edges.into_grouping_map()
      .fold(Vec::<V>::new(), |mut acc, _key, val| { 
        if !acc.contains(&val) { acc.push(val) }
        acc
      });
    BiGraph { edges: map }
  }
}

impl<V> FromStr for BiGraph<V> where V: KeyTraits + FromStr, <V as FromStr>::Err: Debug {
    type Err = Infallible;

    fn from_str(lines: &str) -> Result<Self, Self::Err> { Ok(
      lines.split('\n').map(|l| {
        let (a, b) = l.trim().split_once('-').expect("Unable to find -");
        let a = a.parse::<V>().unwrap();
        let b = b.parse::<V>().unwrap();
        (a, b)
      }).collect::<BiGraph<V>>()
    ) }
}

impl<V: Ord + Display> Display for BiGraph<V> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for (k, vs) in self.edges.iter().sorted() {
      for v in vs {
        if k < v {  // Graph is bidirectional and 
          write!(f, "{}-{}", k , v)?;
        }
      }
    }
    Ok(())
  }
}

// impl Graph<Node> {
//   /// **todo**: Validate that there's no BigCave <-> BigCage connection, otherwise traversal will have a cycle
//   fn all_paths<'a>(&'a self) -> Vec<Vec<&'a Node>> {

//   }
// }

#[cfg(test)]
mod test {
  use super::*;
  use super::Node::*;

  const SAMPLE_GRAPH: &str =
"start-A
start-b
A-c
A-b
b-d
A-end
b-end";

  #[test]
  fn test_parse_graph() {
    let graph: BiGraph<Node> = SAMPLE_GRAPH.parse().unwrap();
    let edges = graph.edges_vec();
    
    let expected = vec![
      (Start, BigCave("A".to_string())),
      (Start, SmallCave("b".to_string())),
      (BigCave("A".to_string()), SmallCave("b".to_string())),
      (BigCave("A".to_string()), SmallCave("c".to_string())),
      (BigCave("A".to_string()), End),
      (SmallCave("b".to_string()), SmallCave("d".to_string())),
      (SmallCave("b".to_string()), End),
    ];
    
    assert_eq!(edges, expected);
  }
}