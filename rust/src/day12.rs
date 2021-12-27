use std::collections::{HashSet, HashMap};
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


enum Tree<N: Copy> {
  Leaf(N),
  Branch(N, Vec<Tree<N>>),
}

impl<N: Copy> Tree<N> {
  fn paths(&self) -> Vec<Vec<N>> {
    match self {
      Tree::Leaf(n) => vec![vec![*n]],
      Tree::Branch(n, branches) => 
        branches.iter()
          .flat_map(|b| b.paths())
          .map(|mut p| { p.insert(0, *n); p })
          .collect_vec()
    }
  }
}

/// **todo**: Look for a well established graph library 
#[derive(Debug, PartialEq, Eq)]
struct BiGraph { edges_map: HashMap<Node, Vec<Node>> }

impl BiGraph {
  fn new(edges: Vec<(Node, Node)>) -> Self {
    Self::validate(&edges);
    BiGraph { edges_map: Self::bidirectional_edges_map(edges) }
  }

  fn from_str(lines: &'static str) -> Self {
    lines.split('\n').map(|l| {
      let (a, b) = l.trim().split_once('-').expect("Unable to find -");
      let a = Node::from_str(a);
      let b = Node::from_str(b);
      (a, b)
    }).collect::<BiGraph>()
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

  fn nodes<'a>(&'a self) -> impl Iterator<Item= &'a Node> {
    self.edges_map
      .iter()
      .flat_map( |(k, vs)| std::iter::once(k).chain(vs) )
      .sorted()
      .dedup()
  }

  fn neighbors<'a>(&'a self, from: &Node) -> impl Iterator<Item= &'a Node> {
    self.edges_map .get(&from).into_iter().flat_map(|v| v.iter())
  }

  fn bidirectional_edges(edges: Vec<(Node, Node)>) -> impl Iterator<Item=(Node, Node)> {
    edges.into_iter().flat_map( |(a, b)| [(a, b), (b, a)] )
  }

  fn bidirectional_edges_map(edges: Vec<(Node, Node)>) -> HashMap<Node, Vec<Node>> {
    Self::bidirectional_edges(edges)
      .into_grouping_map()
      .fold(Vec::<Node>::new(), |mut acc, _k, v| { if ! acc.contains(&v) { acc.push(v); }; acc } )
  }

  fn traverse_all(&self, can_visit_twice: bool) -> Tree<Node> { self.traverse(Node::Start, [Node::Start].into(), can_visit_twice) }

  /// **todo**: Add caching OR dynamic programming from End to Start
  /// **unvisited**: Should only contain SmallCaves
  fn traverse(& self, from: Node, visited: HashSet<Node>, can_visit_twice: bool) -> Tree<Node> {
    if matches!(from, Node::End) { return Tree::Leaf(from) }

    let visited_with = |n: Node| {
      let mut v = visited.clone();
      if matches!(n, Node::SmallCave(_)) { v.insert(n); }
      v
    };

    let branches = self .neighbors(&from)
      .filter(|n| ! matches!(n, Node::Start) ) // Don't want to circle back to Start
      .filter(|n| can_visit_twice || ! visited.contains(n) )
      .map(|&n| self.traverse(n, visited_with(n), can_visit_twice && ! visited.contains(&n) ) )
      .collect_vec();

    Tree::Branch(from, branches)
   }
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

  const SAMPLE_GRAPHS: [&str; 3] = [
"start-A
start-b
A-c
A-b
b-d
A-end
b-end",

"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc",

"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW",
  ];

  #[test]
  fn test_parse_graph() {
    let bi_graph: BiGraph = BiGraph::from_str(SAMPLE_GRAPHS[0]);
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

  #[test]
  fn tests_traverse_all() {
    let bi_graph = BiGraph::from_str(SAMPLE_GRAPHS[0]);
    let all_paths = bi_graph.traverse_all(false).paths();
    let expected = vec![
      vec![Start, BigCave("A"), SmallCave("c"), BigCave("A"), SmallCave("b"), BigCave("A"), End],
      vec![Start, BigCave("A"), SmallCave("c"), BigCave("A"), SmallCave("b"), End],
      vec![Start, BigCave("A"), SmallCave("c"), BigCave("A"), End],
      vec![Start, BigCave("A"), SmallCave("b"), BigCave("A"), SmallCave("c"), BigCave("A"), End],
      vec![Start, BigCave("A"), SmallCave("b"), BigCave("A"), End],
      vec![Start, BigCave("A"), SmallCave("b"), End],
      vec![Start, BigCave("A"), End],
      vec![Start, SmallCave("b"), BigCave("A"), SmallCave("c"), BigCave("A"), End],
      vec![Start, SmallCave("b"), BigCave("A"), End],
      vec![Start, SmallCave("b"), End],
    ];
    assert_eq!(all_paths, expected);
  }

  #[test]
  fn tests_traverse_all_lens() {
    const expected: [[usize; 3]; 2] = [ [10, 19, 226], [36, 103, 3509]];

    for (i, &can_visit_twice) in [false, true].iter().enumerate() {
      for j in 0..3 {
        let bi_graph = BiGraph::from_str(SAMPLE_GRAPHS[j]);
        let all_paths = bi_graph.traverse_all(can_visit_twice).paths();
        assert_eq!(all_paths.len(), expected[i][j]);
      }
    }
  }

  const INPUT: &str =
"kc-qy
qy-FN
kc-ZP
end-FN
li-ZP
yc-start
end-qy
yc-ZP
wx-ZP
qy-li
yc-li
yc-wx
kc-FN
FN-li
li-wx
kc-wx
ZP-start
li-kc
qy-nv
ZP-qy
nv-xr
wx-start
end-nv
kc-nv
nv-XQ";

  #[test]
  fn part1() {
    let bi_graph = BiGraph::from_str(INPUT);
    let paths = bi_graph.traverse_all(false).paths();
    println!("day12 part 1 answer = {}", paths.len());
    assert_eq!(paths.len(), 5874);
  }
  #[test]
  fn part2() {
    let bi_graph = BiGraph::from_str(INPUT);
    let paths = bi_graph.traverse_all(true).paths();
    println!("day12 part 2 answer = {}", paths.len());
    assert_eq!(paths.len(), 153592);
  }
}