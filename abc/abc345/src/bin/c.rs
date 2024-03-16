#![allow(non_snake_case)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut S: Chars
    };

    S.sort();
    let len = S.len();
    let map = run_length_encoding(S);

    if map.len() == 1 {
        println!("{}", 1);
        return;
    }

    let mut count = 0;
    for (_, c) in map.iter() {
        if *c >= 2 {
            count += (c * (c - 1)) / 2;
        }
    }

    count = count.saturating_sub(1);
    let ans = (len * (len - 1) / 2) - count;
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
