use std::borrow::Borrow;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum ParseResult {
  Valid,
  Incomplete,
  Corrupted(char),
}

fn is_open_delimiter(c: char) -> bool { "([{<".contains(c) }

fn are_matching_pair(open: char, close: char) -> bool {
  match (open, close) {
    ('(', ')') | ('[', ']') | ('<', '>') | ('{', '}') => true,
    _ => false,
  }
}

fn parse_line<S: AsRef<str>>(line: S) -> ParseResult {
  let mut stack: Vec<char> = Vec::new();
  for c in line.as_ref().chars() {
    if is_open_delimiter(c) {
      stack.push(c)
    } else {
      match stack.pop() {
        None => return ParseResult::Corrupted(c),
        Some(o) =>  if !are_matching_pair(o, c) { return ParseResult::Corrupted(c) }
      }
    }
  }
  if stack.is_empty() { ParseResult::Valid } else { ParseResult::Incomplete }
}

fn score<R: Borrow<ParseResult>>(results: impl Iterator<Item=R>) -> u64 {
  results.map(|r| match r.borrow() {
    ParseResult::Corrupted(')') => 3,
    ParseResult::Corrupted(']') => 57,
    ParseResult::Corrupted('}') => 1197,
    ParseResult::Corrupted('>') => 25137,
    _ => 0
  }).sum()
}


#[cfg(test)]
mod test {
  use itertools::Itertools;
  use crate::common;

use super::*;
  use super::ParseResult::*;

  #[test]
  fn test_parse_line_valid() {
    const VALID_LINES: [&str; 9] = [
      "{}", "[]", "<>",  "()",
      "([])",
      "{()()()}",
      "<([{}])>",
      "[<>({}){}[([])<>]]",
      "(((((((((())))))))))",
    ];
    for l in VALID_LINES { assert_eq!(parse_line(l), Valid); }
  }

  #[test]
  fn test_parse_line_corrupted() {
    const CORRUPTED_LINES: [&str; 4] = [ "(]", "{()()()>", "(((()))}", "<([]){()}[{}])" ];
    const CORRUPTING_CHARS: [char ; 4] = [']', '>', '}', ')'];
    for (l, c) in CORRUPTED_LINES.iter().zip(CORRUPTING_CHARS) { assert_eq!(parse_line(l), Corrupted(c)); }
  }

  const SAMPLE: &str = 
"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

  #[test]
  fn test_part1_sample() {
    let results = SAMPLE.split('\n').map(parse_line).collect_vec();
    let expected = vec![
      Incomplete,     Incomplete, Corrupted('}'), Incomplete,     Corrupted(')'),
      Corrupted(']'), Incomplete, Corrupted(')'), Corrupted('>'), Incomplete
    ];
    assert_eq!(results, expected);
    assert_eq!(score(results.iter()), 26397);
  }

  #[test]
  fn part1() {
    let results = common::parse::read_lines("../input/day10.txt").map(parse_line);
    let total_score =  score(results);
    println!("day9 part1 answer = {}", total_score);
  }



}