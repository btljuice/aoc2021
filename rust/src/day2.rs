use super::common;

pub fn part1() -> i32 {
    let directions = common::read_lines("../input/day2.txt").map(|s| direction::from(s.as_str()));
    let (x, y) = direction::to_coord(directions);
    x * y
}

pub fn part2() -> i32 {
    let directions = common::read_lines("../input/day2.txt").map(|s| direction::from(s.as_str()));
    let (x, d, _) = directions.fold((0, 0, 0), |(x, d, a), dir| match dir {
        Direction::Forward(dx) => (x + dx, d + a*dx, a),
        Direction::Down(da) => (x, d, a + da),
        Direction::Up(da) => (x, d, a - da),
        });
    x * d
}

enum Direction { Forward(i32), Down(i32), Up(i32) }

/** @todo ANSME: Can an enum have a companion `impl Direction` ? */
mod direction {
    use super::Direction;

    /** @todo Return a Result<...> instead */
    fn ctor(label: &str) -> fn(i32) -> Direction {
        match label.to_lowercase().as_str() {
            "forward" => |x| Direction::Forward(x),
            "down" => |y| Direction::Down(y),
            "up" => |y| Direction::Up(y),
            _ => panic!("Unexpected label. Should be forward|down|up"),
        }
    }

    /** @todo Return a Result<...> instead */
    pub(super) fn from(s: &str) -> Direction {
        s.trim().split_once(' ')
            .map(|(label, amplitude_str)| {
                let amplitude: i32 = amplitude_str.parse().expect("Unable to parse amplitude");
                ctor(label)(amplitude)
            }).expect("Unable to parse direction")
    }

    pub(super) fn to_coord(dirs: impl Iterator<Item=Direction>) -> (i32, i32) {
        dirs.fold((0, 0), |(x, y), dir| match dir {
            Direction::Forward(dx) => (x + dx, y),
            Direction::Down(dy) => (x, y + dy),
            Direction::Up(dy) => (x, y - dy)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::direction;

    const DIRECTIONS: &'static str = "
forward 5
down 5
forward 8
up 3
down 8
forward 2
";

    #[test]
    fn test_to_coord() {
        let directions = DIRECTIONS
            .trim().split('\n')
            .map(|s| direction::from(s));
        let xy = direction::to_coord(directions);
        assert_eq!(xy, (15, -10))
    }
}
