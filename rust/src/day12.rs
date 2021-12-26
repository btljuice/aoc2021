use std::convert::Infallible;
use std::str::FromStr;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Node {
  Start,
  End,
  BigCave(String),
  SmallCave(String),
}

// impl<A> From<Result<A, Infallible>> for A {
//     fn from(r: Result<A, Infallible>) -> Self { r.unwrap() }
// }

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

/// **todo**: Look for a well established graph library 
/// **todo**: Convert to HashMap<V, List[V]> For faster lookup
#[derive(Debug, PartialEq, Eq)]
struct Graph<V> { edges: Vec<(V, V)> }

impl<V> FromIterator<(V, V)> for Graph<V> {
  fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = (V, V)> {
    Graph { edges: iter.into_iter().collect() }
  }
}

impl<V> FromStr for Graph<V> where V: FromStr, <V as FromStr>::Err: Debug {
    type Err = Infallible;

    fn from_str(lines: &str) -> Result<Self, Self::Err> { Ok(
      lines.split('\n').map(|l| {
        let (a, b) = l.trim().split_once('-').expect("Unable to find -");
        let a = a.parse::<V>().unwrap();
        let b = b.parse::<V>().unwrap();
        (a, b)
      }).collect::<Graph<V>>()
      )
    }
}

// impl Graph<Node> {
//   /// **todo**: Validate that there's no BigCave <-> BigCage connection, otherwise traversal will have a cycle
//   fn all_paths(&'a self) -> Vec<Vec<&'a Node>> {

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
    let graph: Graph<Node> = SAMPLE_GRAPH.parse().unwrap();
    let expected: Graph<Node> = Graph { edges: vec![
      (Start, BigCave("A".to_string())),
      (Start, SmallCave("b".to_string())),
      (BigCave("A".to_string()), SmallCave("c".to_string())),
      (BigCave("A".to_string()), SmallCave("b".to_string())),
      (SmallCave("b".to_string()), SmallCave("d".to_string())),
      (BigCave("A".to_string()), End),
      (SmallCave("b".to_string()), End),
    ] };
    
      assert_eq!(graph, expected);
  }
}