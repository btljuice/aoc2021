use super::common;

pub fn part1() -> i32 {
    let lines = common::read_lines("../input/day3.txt");
    0
}

pub(self) fn to_bitvec(s: &str) -> Vec<bool> {
    s.trim().chars().map(|c| match c {
        '0' => false,
        '1' => true,
        _ => panic!("String = {} should only contain 0 or 1", s)
    }).collect()
}

// ANSME How to provide an Iterator<Item=&[bool]> or Iterator<Item=&Vec<bool>>
pub(self) fn most_common_bits(
    bitvecs: impl Iterator<Item=Vec<bool>>,
    expected_size: usize,
) -> Vec<bool> {
    // ANSME: How to convert a .fold and ensure `weights` is moved across fold inner calls ?
    // There's good chance that it's already the case w/ the default implementation.
    let mut weights = vec![0; expected_size];
    for bits in bitvecs {
        // Resizes weights if too small
        if bits.len() > weights.len() { weights.resize_with(bits.len(), || 0); };
        for (i, &b) in bits.iter().enumerate() {
            weights[i] += if b { 1 } else { -1 };
        }
    }
    weights.iter().map(|w| *w >= 0).collect()
}

#[cfg(test)]
mod tests {
    use crate::day3::most_common_bits;
    use super::to_bitvec;

    const STATES: &'static str = "
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

    #[test]
    fn test_to_bitvec() {
        assert_eq!(to_bitvec("010110"), vec![false, true, false, true, true, false])
    }

    #[test]
    fn test_most_common_bits() {
        let bitvecs = STATES.trim().split('\n')
            .map(to_bitvec);
        let common_bits = most_common_bits(bitvecs, 5);
        assert_eq!(common_bits, vec![true, false, true,  true, false])
    }
}
