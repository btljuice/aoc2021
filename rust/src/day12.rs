use std::cmp::Ordering;
use std::collections::{HashSet, HashMap};
use std::convert::Infallible;
use std::str::FromStr;
use std::fmt::Debug;
use std::hash::Hash;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Node {
  Start,
  SmallCave(&'static str),
  BigCave(&'static str),
  End,
}

impl Node {
  fn from_str(s: &'static str) -> Node {
    match s {
      "start" => Node::Start,
      "end" => Node::End,
      _ if s.chars().all(|c| c.is_lowercase()) => Node::SmallCave(s),
      _ => Node::BigCave(s),
    }
  }
}

trait KeyTraits: Hash + Eq + Debug {}
impl<K> KeyTraits for K where K: Hash + Eq + Debug {}

/// **todo**: Look for a well established graph library 
#[derive(Debug, PartialEq, Eq)]
struct BiGraph { edges_map: HashMap<Node, Vec<Node>> }

impl BiGraph {
  fn new(edges: Vec<(Node, Node)>) -> Self {
    Self::validate(&edges);
    BiGraph { edges_map: Self::bidirectional_edges_map(edges) }
  }

  /// Validates that there's no `BigCave` <-> `BigCage` edge, otherwise exhaustive path traversal will loop infinitely.
  fn validate(edges: &Vec<(Node, Node)>) {
    for e in edges {
      assert!(
        ! matches!(e, (Node::BigCave(_), Node::BigCave(_))),
        "No BigCave-BigCave edge can exist, as they would create cycles"
      );
    }
  }

  fn nodes<'a> (&'a self) -> impl Iterator<Item= &'a Node> {
    self.edges_map
      .iter()
      .flat_map( |(k, vs)| std::iter::once(k).chain(vs) )
      .sorted()
      .dedup()
  }

  fn bidirectional_edges<'a>(edges: &'a Vec<(Node, Node)>) -> impl Iterator<Item=(&'a Node, &'a Node)> {
    edges.iter().flat_map( |(a, b)| [(a, b), (b, a)] )
  }

  fn bidirectional_edges_map(edges: Vec<(Node, Node)>) -> HashMap<Node, Vec<Node>> {
    Self::bidirectional_edges(&edges)
      .map(|(&a, &b)| (a, b))
      .into_grouping_map()
      .fold(Vec::<Node>::new(), |mut acc, _k, v| { if !acc.contains(&v) { acc.push(v); }; acc } )
  }

  fn from_str(lines: &'static str) -> Self {
    lines.split('\n').map(|l| {
      let (a, b) = l.trim().split_once('-').expect("Unable to find -");
      let a = Node::from_str(a);
      let b = Node::from_str(b);
      (a, b)
    }).collect::<BiGraph>()
  }


  // fn traverse_all(&'a self) -> Vec<Vec<&'a Node>> {
  //   self.validate();
  //   self.traverse(& Node::Start)
  // }
  // /// **todo**: Add caching OR dynamic programming from End to Start
  // fn traverse<'a>(&'a self, root: &'a Node) -> Vec<Vec<&'a Node>> {
  //   if matches!(root, End) { return Vec::new() }

  //   let edges_map: HashMap<_, _> = self.bidirectional_edges_map();
  //   todo!()
  // }
}

impl FromIterator<(Node, Node)> for BiGraph {
  fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = (Node, Node)> {
    BiGraph::new(iter.into_iter().collect())
  }
}


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
    let bi_graph: BiGraph = BiGraph::from_str(SAMPLE_GRAPH);
    let assoc_list: Vec<(Node, Vec<Node>)> = bi_graph.edges_map.into_iter()
      .map( |(k, mut vs)| { vs.sort(); (k, vs) } )
      .sorted()
      .collect_vec();
    let expected: Vec<(Node, Vec<Node>)> = vec![
      ( Start,          vec![ SmallCave("b"), BigCave("A") ] ),
      ( SmallCave("b"), vec![ Start, SmallCave("d"), BigCave("A"), End ] ),
      ( SmallCave("c"), vec![ BigCave("A") ] ),
      ( SmallCave("d"), vec![ SmallCave("b") ] ),
      ( BigCave("A"),   vec![ Start, SmallCave("b"), SmallCave("c"), End ] ),
      ( End,            vec![ SmallCave("b"), BigCave("A") ]),
    ];

    assert_eq!(assoc_list, expected);
    
  }
}