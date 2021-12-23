use std::borrow::Borrow;

use itertools::Itertools;
use rdxsort::RdxSort;

#[derive(PartialEq, Eq, Clone, Debug)]
enum ParseResult {
  Valid,
  Incomplete(String), // Missing delimiters
  Corrupted(char),
}

fn match_closing_delimiter(open: char) -> Option<char> {
  match open {
    '(' => Some(')'),
    '[' => Some(']'),
    '{' => Some('}'),
    '<' => Some('>'),
    _ => None
  }
}


fn parse_line<S: AsRef<str>>(line: S) -> ParseResult {
  let mut stack: Vec<char> = Vec::new();
  for c in line.as_ref().chars() {
    if let Some(close) = match_closing_delimiter(c) { stack.push(close); }
    else {
      match stack.pop() {
        None => return ParseResult::Corrupted(c),
        Some(close_expected) => if c != close_expected { return ParseResult::Corrupted(c) }
      }
    }
  }

  if stack.is_empty() { 
    ParseResult::Valid 
  } else {
    ParseResult::Incomplete(stack.into_iter().rev().collect())
  }
}

fn corrupted_score<R: Borrow<ParseResult>>(results: impl Iterator<Item=R>) -> u64 {
  results.map(|r| match r.borrow() {
    ParseResult::Corrupted(')') => 3,
    ParseResult::Corrupted(']') => 57,
    ParseResult::Corrupted('}') => 1197,
    ParseResult::Corrupted('>') => 25137,
    _ => 0
  }).sum()
}

fn incomplete_score<R: Borrow<ParseResult>>(results: impl Iterator<Item=R>) -> u64 {
  fn pt(c: char) -> u64 { match c { ')' => 1, ']' => 2, '}' => 3, '>' => 4, _ => panic!("Unexpected character") } }

  let mut scores = results.filter_map(|r| match r.borrow() {
    ParseResult::Incomplete(completion) => Some(completion.chars().fold(0u64,|sum, c| sum*5 + pt(c))),
    _ => None
  }).collect_vec();

  scores.rdxsort();

  scores[scores.len()/2]
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
      Incomplete("}}]])})]".to_string()),
      Incomplete(")}>]})".to_string()),
      Corrupted('}'),
      Incomplete("}}>}>))))".to_string()),
      Corrupted(')'),
      Corrupted(']'),
      Incomplete("]]}}]}]}>".to_string()),
      Corrupted(')'),
      Corrupted('>'),
      Incomplete("])}>".to_string()),
    ];
    assert_eq!(results, expected);
    assert_eq!(corrupted_score(results.iter()), 26397);
    assert_eq!(incomplete_score(results.iter()), 288957);
  }

  #[test]
  fn part1() {
    let results = common::parse::read_lines("../input/day10.txt").map(parse_line).collect_vec();
    let corrupted =  corrupted_score(results.iter());
    let incomplete = incomplete_score(results.iter());
    println!("day10 part1 answer = {}", corrupted);
    println!("day10 part1 answer = {}", incomplete);

    assert_eq!(corrupted, 358737);
    assert_eq!(incomplete, 4329504793);

  }



}