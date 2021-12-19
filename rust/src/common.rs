use std::borrow::Borrow;
use std::collections::HashMap;
use std::hash::Hash;
use std::fs::File;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;
use std::fmt::Debug;
use std::cmp::Ord;
use std::ops::{Add, Div, Mul, Sub};
use itertools::Itertools;

/** @todo convert return type to Result<impl Iterator<Item=String>, {Error}> on first error */
pub fn read_lines(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|r| r.unwrap())
}

pub fn read_comma_separated<T>(path: impl AsRef<Path>) -> Vec<T>
 where T: FromStr,
       <T as FromStr>::Err: Debug {
    let content = fs::read_to_string(path.as_ref()).unwrap();
    content.split(',').map(|s| s.parse::<T>().expect("Unable to parse T")).collect()
}

/// ANSME: Is there a way to generically define K to either be a value or a ref, or we need another function/signature?
///     - In some cases like the following, it might be cheap to return a value of K.
///     - In others it might be expensive to to copy K (mem and/or cpu) and we'd want to return a HashMap<&K, u64>
pub fn freq_count<Q, T, K>(ite: impl Iterator<Item=Q>, f: fn(&T) -> K) -> HashMap<K, u64> 
    where Q: Borrow<T>, K: Eq + Hash {
    let mut freqs: HashMap<K, u64> = HashMap::new();
    for q in ite {
        let k = f(q.borrow());
        *freqs.entry(k).or_default() += 1;
    }
    freqs
}

pub fn delta<N: Ord + Sub<Output=N>>(a: N, b: N) -> N { if a > b { a - b } else { b - a } }

pub fn sum_n<N>(n: N) -> N
    where N: Add<Output=N> + Mul<Output=N> + Div<Output=N> + From<u8> + Copy
{
    n * (n + 1.into() ) / 2.into()
}
