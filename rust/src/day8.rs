use std::{collections::HashMap, convert::TryInto};
use itertools::Itertools;

use super::common;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
enum Signal { A=0, B=1, C=2, D=3, E=4, F=5, G=6 }

const SIGNALS: [Signal; 7] = [
    Signal::A, 
    Signal::B,
    Signal::C,
    Signal::D,
    Signal::E,
    Signal::F,
    Signal::G
];

type Digit = Vec<Signal>;
type Rosetta = [Signal; 7];


/// The following uses a deterministic way to find the mapping between the wiring -> light-led signals
/// ```
/// d |       leds    | nb leds
/// ---------------------------
/// 0 | a b c   e f g | 6
/// 1 |     c     f   | 2 *
/// 2 | a   c d e   g | 5
/// 3 | a   c d   f g | 5
/// 4 |   b c d   f   | 4 *
/// 5 | a b   d   f g | 5
/// 6 | a b   d e f g | 6
/// 7 | a   c     f   | 3 *
/// 8 | a b c d e f g | 7 *
/// 9 | a b c d   f g | 6
/// ---------------------------
///     8 6 8 7 4 9 7 
///       *     * *
/// b: 6 occurences in all 10 digits
/// e: 4 "
/// f: 9 "
/// c: find number w/ 2 leds. remove     f
/// d: "            " 4 leds. remove b c f
/// a: "            " 3 leds. remove   c f
/// g: "            " 7 leds. remove a b c d e f
///    OR remaining led
/// ```
/// 
/// **TODO**: 
/// - Validate that `ten_digits` abide to the structure above. Otherwise it is an invalid input.
/// - Maybe convert digits representation to u8 or a bitvec
/// - Flagging / discovering leds could be done w/ & | bit-wise operators
/// 
fn signal_rosetta(ten_digits: &[Digit; 10]) -> Rosetta {
    let led_freqs: HashMap<Signal, u64> = common::collections::freq_count(ten_digits.iter().flatten().map(|&s| s));

    // Exploratory choice of the moment: Prefer a closure over a macro, to minimize macro usages. 
    // - Don't know what is the idiomatic way in Rust. Should I prefer a macro or clojure in general for readability
    //   and debugging? Are macro are harder to debug than closures and vice-versa. 
    // - Exploring the clojure way at the moment.
    // Note: I miss being able to create closure w/ the function definition syntax as in Scala, 
    //       In Rust function  are pure and don't capture a lexical scope (they have to be unstateful).
    let led_with_freq = |n: u64| { led_freqs.iter().find(|(_, &f)| f == n).expect(format!("Unable to find a led w/ {} occurences", n).as_str()) };
    let digit_with_leds = |n: usize| { ten_digits.iter().find(|leds| leds.len() == n).expect(format!("Unable to find digit w/ {} leds", n).as_str()) };

    fn remaining_led(digit: &Digit, to_remove: &[Signal]) -> Signal {
        // Assumes there's only one
        *digit.iter().find(|led| ! to_remove.iter().contains(led))
            .expect(format!("Unable remaining led for digit= {:#?}, while removing {:?}", digit, to_remove).as_str())
    }

    let (&f, _) = led_with_freq(9);
    let (&e, _) = led_with_freq(4);
    let (&b, _) = led_with_freq(6);

    let one   = digit_with_leds(2);
    let c = remaining_led(one, &[f]);

    let four  = digit_with_leds(4);
    let d = remaining_led(four, &[b, c, f]);

    let seven = digit_with_leds(3);
    let a = remaining_led(seven, &[c, f]);

    let g = remaining_led(&SIGNALS.to_vec(), &[a, b, c, d, e, f]);

    [a, b, c, d, e, f, g]
}
 
fn translate_digit(digit: &Digit, rosetta: &Rosetta) -> u8 {
    // Start by matching on length of lens
    match digit.len() {
        2 => 1,
        4 => 4,
        3 => 7,
        7 => 8,
        5 if digit.contains(&rosetta[Signal::E as usize]) => 2,
        5 if digit.contains(&rosetta[Signal::B as usize]) => 5,
        5 => 3,
        6 if !digit.contains(&rosetta[Signal::D as usize]) => 0,
        6 if !digit.contains(&rosetta[Signal::C as usize]) => 6,
        6 if !digit.contains(&rosetta[Signal::E as usize]) => 9,
        _ => panic!("Impossible digit {:#?}", digit)
    }
}

fn translate_digits<'a>(digits: impl Iterator<Item=&'a Digit>, rosetta: &Rosetta) -> Vec<u8> {
    let mut translated = Vec::<u8>::new();
    for digit in digits { translated.push(translate_digit(digit, &rosetta)); }
    translated
}

fn parse_digit(s: &str) -> Digit {
    s.chars().into_iter().map(|c| match c {
        'a' => Signal::A,
        'b' => Signal::B,
        'c' => Signal::C,
        'd' => Signal::D,
        'e' => Signal::E,
        'f' => Signal::F,
        'g' => Signal::G,
         _ => panic!("Unknown character {}", c)
    }).collect_vec()
}

fn translate_digits_from_file(filename: &str) -> impl Iterator<Item=Vec<u8>> {
    let lines = common::parse::read_lines(filename);
    lines
      .enumerate()
      .map( |(i, l)| {
        let (s0, s1) = l.split_once('|').expect(format!("Unable to find | on line {}", i).as_str());
        let ten_digits: [Digit; 10] = s0.split_whitespace().map(parse_digit).collect_vec().try_into().unwrap();
        let rosetta: Rosetta = signal_rosetta(&ten_digits);
        let digits: Vec<Digit> = s1.split_whitespace().map(parse_digit).collect_vec();
        digits.iter().map(|d| translate_digit(&d, &rosetta)).collect_vec()
      })
}

fn translate_numbers_from_file(filename: &str) -> impl Iterator<Item=u32> {
    let numbers = translate_digits_from_file(filename);
    numbers.map(|digits| digits.iter().fold(0u32, |acc, &d| acc*10 + u32::from(d)))
}

#[cfg(test)]
mod test {
    use super::*;
    use super::Signal::*;


    const TEN_DIGITS: [&[Signal]; 10] = [ 
                                 // Translated
        &[B, E],                 // 1: C, F
        &[C, F, B, E, G, A, D],  // 8: 
        &[C, B, D, G, E, F],     // 9: [D, C, A, B, F, G]
        &[F, G, A, E, C, D],     // 6: [G, B, E, F, D, A]
        &[C, G, E, B],           // 4: [D, B, F, C]
        &[F, D, C, G, E],        // 5: [G, A, D, B, F]
        &[A, G, E, B, F, D],     // 0: [E, B, F, C, G, A]
        &[F, E, C, D, B],        // 3: [G, F, D, A, C]
        &[F, A, B, C, D],        // 2: [G, E, C, D, A] 
        &[E, D, B],              // 7: [F, A, C]
    ];

    fn ten_digits() -> [Digit; 10] { TEN_DIGITS.map(|d| d.to_vec()) }

    #[test]
    fn test_signal_rosetta() {
        let rosetta = signal_rosetta(&ten_digits());
        assert_eq!(rosetta, [D, G, B, C, A, E, F]);
    }

    #[test]
    fn test_translate_digit() {
        let digits: [Digit; 10] = ten_digits();
        let rosetta = signal_rosetta(&digits);

        let translated = translate_digits(digits.iter(), &rosetta);

        assert_eq!(translated, vec![1, 8, 9, 6, 4, 5, 0, 3, 2, 7]);
    }

    #[test]
    fn test_parse_digit() {
        let digit = parse_digit("abcd");
        assert_eq!(digit, vec![A, B, C, D]);
    }

    #[test]
    fn test_sample_part1() {
        let digits = translate_digits_from_file("../input/day8_sample.txt").flatten();
        let count = digits.filter(|&n| n == 1 || n == 4 || n == 7 || n == 8).count();
        assert_eq!(count, 26);
    }

    #[test]
    fn test_part1() {
        let digits = translate_digits_from_file("../input/day8.txt").flatten();
        let count = digits.filter(|&n| n == 1 || n == 4 || n == 7 || n == 8).count();
        println!("part 1 answer = {}", count);
        assert_eq!(count, 539);
    }

    #[test]
    fn test_sample_part2() {
        let numbers = translate_numbers_from_file("../input/day8_sample.txt").collect_vec();
        assert_eq!(numbers, vec![ 8394, 9781, 1197, 9361, 4873, 8418, 4548, 1625, 8717, 4315 ]);
        assert_eq!(numbers.iter().sum::<u32>(), 61229);
    }

    #[test]
    fn test_part2() {
        let numbers = translate_numbers_from_file("../input/day8.txt");
        let sum: u32 = numbers.sum();
        println!("part 2 answer = {}", sum);
        assert_eq!(sum, 1084606);
    }

}
