use std::collections::HashMap;
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
fn signal_rosetta(ten_digits: &[Vec<Signal>; 10]) -> [Signal; 7] {
    let led_freqs: HashMap<Signal, u64> = common::collections::freq_count(ten_digits.iter().flatten().map(|&s| s));

    // Exploratory choice of the moment: Prefer a closure over a macro, to minimize macro usages. 
    // - Don't know what is the idiomatic way in Rust. Should I prefer a macro or clojure in general for readability
    //   and debugging? Are macro are harder to debug than closures and vice-versa. 
    // - Exploring the clojure way at the moment.
    // Note: I miss being able to create closure w/ the function definition syntax as in Scala, 
    //       In Rust function  are pure and don't capture a lexical scope (they have to be unstateful).
    let led_with_freq = |n: u64| { led_freqs.iter().find(|(_, &f)| f == n).expect(format!("Unable to find a led w/ {} occurences", n).as_str()) };
    let digit_with_leds = |n: usize| { ten_digits.iter().find(|leds| leds.len() == n).expect(format!("Unable to find digit w/ {} leds", n).as_str()) };

    fn remaining_led(digit: &Vec<Signal>, to_remove: &[Signal]) -> Signal {
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

#[cfg(test)]
mod test {
    use super::*;
    use super::Signal::*;
    
    const TEN_DIGITS: [&[Signal]; 10] = [ 
        &[B, E],
        &[C, F, B, E, G, A, D],
        &[C, B, D, G, E, F],
        &[F, G, A, E, C, D],
        &[C, G, E, B],
        &[F, D, C, G, E],
        &[A, G, E, B, F, D],
        &[F, E, C, D, B],
        &[F, A, B, C, D],
        &[E, D, B],
    ];

    #[test]
    fn test_signal_rosetta() {
        let rosetta = signal_rosetta(&TEN_DIGITS.map(|d| d.to_vec()));
        assert_eq!(rosetta, [D, G, B, C, A, E, F]);
    }
}
