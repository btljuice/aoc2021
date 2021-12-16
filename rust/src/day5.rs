use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use itertools::Itertools;
use std::collections::HashMap;

type Number = u32;

fn draw_lines<'a>(lines: impl Iterator<Item=&'a Line>) -> HashMap<Point, u32> {
    let mut freq_count = HashMap::<Point, u32>::new();

    for l in lines {
        if !l.is_horizontal() && !l.is_vertical() { continue; }

        let ((min_x, min_y), (max_x,max_y)) = l.bounding_box().to_tuples();
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                println!("Adding {}, {}", x, y);
                freq_count.entry(Point{x, y}).and_modify(|n| *n += 1).or_insert(1);
            }
        }
    }

    freq_count
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash) ]
struct Point { x: Number, y: Number }

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Point {
    fn bounding_box(&self, rhs: &Point) -> (Point, Point) {
        let min_pt = Point { x: Number::min(self.x, rhs.x), y: Number::min(self.y, rhs.y) };
        let max_pt = Point { x: Number::max(self.x, rhs.x), y: Number::max(self.y, rhs.y) };
        (min_pt, max_pt)
    }
}

#[derive(Copy, Clone, Debug, PartialEq) ]
struct Line { p0: Point, p1: Point }
impl Line {
    fn from_tuples(p0: (Number, Number), p1: (Number, Number)) -> Line {
        Line { p0: Point { x: p0.0, y : p0.1 }, p1: Point { x: p1.0, y: p1.1 } }
    }

    fn bounding_box(&self) -> Line {
        let (min_pt, max_pt) = self.p0.bounding_box(&self.p1);
        Line { p0: min_pt, p1: max_pt }
    }

    fn to_tuples(&self) -> ((Number, Number), (Number, Number)) {
        ((self.p0.x, self.p0.y), (self.p1.x, self.p1.y))
    }

    fn is_vertical(&self) -> bool   { self.p0.x == self.p1.x }
    fn is_horizontal(&self) -> bool { self.p0.y == self.p1.y }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PointParseError {
    MissingComma,
    InvalidNumber(&'static str),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LineParseError {
    MissingArrow,
    InvalidPoint(PointParseError, &'static str),
}

#[derive(Debug, PartialEq)]
struct WithLineNumber<T: Debug> { line_no: usize, value: T }

#[derive(Debug, PartialEq)]
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

    const LINES_STR: &str = "
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

    macro_rules! line { ($x0:literal, $y0:literal, $x1:literal, $y1:literal) => {
            Line { p0: Point { x: $x0, y: $y0 }, p1: Point { x: $x1, y: $y1 } }
        } }

    const LINES: [Line; 10] = [
        line!(0, 9, 5, 9),
        line!(8, 0, 0, 8),
        line!(9, 4, 3, 4),
        line!(2, 2, 2, 1),
        line!(7, 0, 7, 4),
        line!(6, 4, 2, 0),
        line!(0, 9, 2, 9),
        line!(3, 4, 1, 4),
        line!(0, 0, 8, 8),
        line!(5, 5, 8, 2),
    ];

    #[test]
    fn test_parse_line_segments() {
        let lines = parse_lines(LINES_STR.trim().split('\n')).unwrap();
        assert_eq!(lines.as_slice(), LINES);
    }

    #[test]
    fn test_parse_errors() {
        const ERROR_STR: [&str; 4] = [
            "1 -> 2,3",   // Missing comma
            "1,2 -> 3,4",  // Good line !
            "1,2 -> 3,a", // Invalid Number
            "1,2 3,4",    // Missing Arrow
        ];
        let expected: LineParseErrors = LineParseErrors { errors: vec![
            WithLineNumber { line_no: 1, value: LineParseError::InvalidPoint(PointParseError::MissingComma, "p0") },
            WithLineNumber { line_no: 3, value: LineParseError::InvalidPoint(PointParseError::InvalidNumber("y"), "p1") },
            WithLineNumber { line_no: 4, value: LineParseError::MissingArrow },
        ] };

        let errors = parse_lines(ERROR_STR.iter().map(|s|*s)).unwrap_err();
        assert_eq!(errors, expected);
    }

    #[test]
    fn test_intersect_ver_hor_lines() {
        macro_rules! p { ($x:literal, $y:literal) => { Point { x:$x, y:$y } } }
        const EXPECTED: [(Point, u32); 21] = [
            (p!(2, 1), 1), (p!(2, 2), 1),
            (p!(7, 0), 1), (p!(7, 1), 1), (p!(7, 2), 1), (p!(7, 3), 1),
            (p!(1, 4), 1), (p!(2, 4), 1), (p!(3, 4), 2), (p!(4, 4), 1), (p!(5, 4), 1), (p!(6, 4), 1), (p!(7, 4), 2), (p!(8, 4), 1), (p!(9, 4), 1),
            (p!(0, 9), 2), (p!(1, 9), 2), (p!(2, 9), 2), (p!(3, 9), 1), (p!(4, 9), 1), (p!(5, 9), 1),
        ];
        let draw_board = draw_lines(LINES.iter());
        for (k, v) in &EXPECTED {
            assert_eq!(draw_board.get(k), Some(v), "Entry {} mismatch", k)
        }
    }
}
