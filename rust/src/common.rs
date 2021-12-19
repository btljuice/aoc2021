pub(crate) mod parse { 
    use std::io::{self, BufRead};
    use std::path::Path;
    use std::str::FromStr;
    use std::fmt::Debug;
    use std::fs::File;
    use std::fs;

    /** @todo convert return type to Result<impl Iterator<Item=String>, {Error}> on first error */
    pub fn read_lines(filename: &str) -> impl Iterator<Item=String> {
        let file = File::open(filename).unwrap();
        io::BufReader::new(file).lines().map(|r| r.unwrap())
    }

    pub fn read_comma_separated<T>(path: impl AsRef<Path>) -> Vec<T>
    where T: FromStr, <T as FromStr>::Err: Debug {
        let content = fs::read_to_string(path.as_ref()).unwrap();
        content.split(',').map(|s| s.parse::<T>().expect("Unable to parse T")).collect()
    }
}

pub(crate) mod collections {
    use std::collections::HashMap;
    use std::hash::Hash;

    pub fn freq_count<K>(ite: impl Iterator<Item=K>) -> HashMap<K, u64>
    where K: Eq + Hash {
        let mut freqs: HashMap<K, u64> = HashMap::new();
        for k in ite { *freqs.entry(k).or_default() += 1; }
        freqs
    }
}


pub(crate) mod math {
    use std::cmp::Ord;
    use std::ops::{Add, Div, Mul, Sub};
     
    pub fn delta<N>(a: N, b: N) -> N 
    where N: Ord + Sub<Output=N> {
        if a > b { a - b } else { b - a } 
    }

    pub fn sum_n<N>(n: N) -> N
    where N: Add<Output=N> + Mul<Output=N> + Div<Output=N> + From<u8> + Copy {
        n * (n + 1.into() ) / 2.into()
    }
}
