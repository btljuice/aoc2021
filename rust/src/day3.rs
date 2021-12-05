use std::fs::read;
use super::common;

pub fn part1() -> u32 {
    let bitvecs =
        common::read_lines("../input/day3.txt")
            .map(|l| to_bitvec(l.as_str()));
    power_consumption(bitvecs)
}

pub(self) fn power_consumption(bits: impl Iterator<Item=Vec<bool>>) -> u32 {
    let most_common = most_common_bits(bits, 12);
    let gamma = to_integer(&most_common);
    let epsilon = to_integer(invert_bits(&most_common).as_slice());
    println!("gamma = {}, epsilon = {}", gamma, epsilon);
    gamma * epsilon
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
    let mut weights = Vec::<i32>::new();
    weights.reserve_exact(expected_size);
    for bits in bitvecs {
        // Resizes weights if too small
        if bits.len() > weights.len() { weights.resize_with(bits.len(), || 0); };
        for (i, &b) in bits.iter().enumerate() {
            weights[i] += if b { 1 } else { -1 };
        }
    }
    weights.iter().map(|w| *w >= 0).collect() // When there's equality, select 1
}

pub(self) fn invert_bits(bits: &[bool]) -> Vec<bool> { bits.iter().map(|x| !x).collect() }

pub(self) fn to_integer(bits: &[bool]) -> u32 {
    bits.iter().fold(0, |acc, &b| (acc<<1) + u32::from(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ANSME: How can I go further and pre-compute these values into [bool] at compile time
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
    fn test_power_consumption() {
        let bitvecs = STATES.trim().split('\n').map(to_bitvec);
        assert_eq!(power_consumption(bitvecs), 198)
    }

    #[test]
    fn test_to_bitvec() {
        assert_eq!(to_bitvec("010110"), vec![false, true, false, true, true, false])
    }

    #[test]
    fn test_most_common_bits() {
        // ANSME How to convert from a Iterator<Item=Vec<bool>> to a Iterator<Item=&Vec<bool>>
        // or Iterator<Item=&[bool]>, and make it as a parameter to most_common_bits
        let bitvecs = STATES.trim().split('\n')
            .map(to_bitvec);
        let common_bits = most_common_bits(bitvecs, 5);
        assert_eq!(common_bits, vec![true, false, true,  true, false])
    }

    #[test]
    fn test_invert_bits() {
        let n =     to_bitvec("01010");
        let inv_n = to_bitvec("10101");
        assert_eq!(invert_bits(n.as_slice()), inv_n)
    }

    #[test]
    fn test_to_integer() {
        let n = to_integer(&vec![true, false , true, false]);
        assert_eq!(n, 10)
    }
}
