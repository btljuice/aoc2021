use std::collections::HashMap;
use std::hash::Hash;
use std::fs::File;
use std::io::{self, BufRead};

/** @todo convert return type to Result<impl Iterator<Item=String>, {Error}> on first error */
pub fn read_lines(filename: &str) -> impl Iterator<Item=String> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file).lines().map(|r| r.unwrap())
}

/// ANSME: Is there a way to generically define K to either be a value or a ref, or we need another function/signature?
///     - In some cases like the following, it might be cheap to return a value of K.
///     - In others it might be expensive to to copy K (mem and/or cpu) and we'd want to return a HashMap<&K, usize>
pub fn freq_count<'a, T, K>(ite: impl Iterator<Item=&'a T>, f: fn(&T) -> K) -> HashMap<K, usize> 
    where T: 'a, K: Eq + Hash {
    let mut freqs: HashMap<K, usize> = HashMap::new();
    for t in ite {
        let k = f(t);
        *freqs.entry(k).or_default() += 1;
    }
    freqs
}
