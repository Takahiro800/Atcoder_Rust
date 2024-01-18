#![allow(non_snake_case)]
use proconio::input;
use proconio::marker::*;

fn main() {
    input! {
        _N: usize,
        S: Bytes,
    };

    let z = run_length_encoding(S);
    let mut count = vec![0; 26];

    for (c, k) in z {
        let c = (c - b'a') as usize;
        count[c] = count[c].max(k);
    }

    let ans = count.iter().sum::<usize>();
    println!("{}", ans);
}

fn run_length_encoding<T: Eq>(a: Vec<T>) -> Vec<(T, usize)> {
    let mut a = a.into_iter().map(|a| (a, 1)).collect::<Vec<_>>();
    a.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });

    a
}
