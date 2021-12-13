use std::fmt::Debug;
use std::str::FromStr;
use itertools::Itertools;

type Number = u32;

#[derive(Copy, Clone, Debug, PartialEq) ]
struct Point { x: Number, y: Number }

#[derive(Copy, Clone, Debug, PartialEq) ]
struct Line { p0: Point, p1: Point }
impl Line {
    fn from(p0: (Number, Number), p1: (Number, Number)) -> Line {
        Line { p0: Point { x: p0.0, y : p0.1 }, p1: Point { x: p1.0, y: p1.1 } }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum PointParseError {
    MissingComma,
    InvalidNumber(&'static str),
}

#[derive(Copy, Clone, Debug)]
pub enum LineParseError {
    MissingArrow,
    InvalidPoint(PointParseError, &'static str),
}

#[derive(Debug)]
struct WithLineNumber<T: Debug> { line_no: usize, value: T }

#[derive(Debug)]
struct LineParseErrors { errors: Vec<WithLineNumber<LineParseError>> }

fn parse_lines<'a>(lines: impl Iterator<Item=&'a str>) -> Result<Vec<Line>, LineParseErrors> {
    let (lines, errors): (Vec<_>, Vec<_>) = lines.enumerate()
        .map(|(i, l)| l.parse::<Line>().map_err(|e| WithLineNumber { line_no: i+1, value: e }))
        .partition_result();
    if !errors.is_empty() { Err(LineParseErrors{ errors }) }
    else { Ok(lines) }
}


//// Converters

impl FromStr for Point {
    type Err = PointParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (str_x, str_y) = s.split_once(',').ok_or(PointParseError::MissingComma)?;
        let x = str_x.trim().parse::<Number>().map_err(|_| PointParseError::InvalidNumber("x"))?;
        let y = str_y.trim().parse::<Number>().map_err(|_| PointParseError::InvalidNumber("y"))?;
        Ok(Point { x, y })
    }
}

impl FromStr for Line {
    type Err = LineParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (str_p0, str_p1) = s.split_once("->").ok_or(LineParseError::MissingArrow)?;
        let p0 = str_p0.trim().parse::<Point>().map_err(|e| LineParseError::InvalidPoint(e, "p0"))?;
        let p1 = str_p1.trim().parse::<Point>().map_err(|e| LineParseError::InvalidPoint(e, "p1"))?;
        Ok(Line { p0, p1 })
    }
}

#[cfg(test)]
pub(self) mod tests {
    use super::*;

    const LINE_SEGMENTS_STR: &'static str = "
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

    #[test]
    fn test_parse_line_segments() {
        let lines = parse_lines(LINE_SEGMENTS_STR.trim().split('\n')).unwrap();
        assert_eq!(lines[0], Line::from((0, 9), (5, 9)));
        assert_eq!(lines[9], Line::from((5, 5), (8, 2)));
    }
}
