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
    const LINES: [Line; 10] = [
        Line { p0: Point { x: 0, y: 9 }, p1: Point { x: 5, y: 9 } },
        Line { p0: Point { x: 8, y: 0 }, p1: Point { x: 0, y: 8 } },
        Line { p0: Point { x: 9, y: 4 }, p1: Point { x: 3, y: 4 } },
        Line { p0: Point { x: 2, y: 2 }, p1: Point { x: 2, y: 1 } },
        Line { p0: Point { x: 7, y: 0 }, p1: Point { x: 7, y: 4 } },
        Line { p0: Point { x: 6, y: 4 }, p1: Point { x: 2, y: 0 } },
        Line { p0: Point { x: 0, y: 9 }, p1: Point { x: 2, y: 9 } },
        Line { p0: Point { x: 3, y: 4 }, p1: Point { x: 1, y: 4 } },
        Line { p0: Point { x: 0, y: 0 }, p1: Point { x: 8, y: 8 } },
        Line { p0: Point { x: 5, y: 5 }, p1: Point { x: 8, y: 2 } },
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
}
