#![allow(non_snake_case)]
use proconio::input;
use std::usize;

fn main() {
    input! {
        N: usize,
        AB: [(usize,usize);N],
    };

    let mut A: Vec<usize> = AB.iter().map(|(a, _)| *a).collect();
    let mut B: Vec<usize> = AB.iter().map(|(_, b)| *b).collect();
    A.sort();
    B.sort();
    // println!("{:?}", A);
    // println!("{:?}", B);
    let map_a = run_length_encoding(A);
    let map_b = run_length_encoding(B);
    println!("{:?}", map_a);
    println!("{:?}", map_b);

    let count_a: usize = map_a.iter().map(|(_, v)| v / 2).sum();
    let count_b: usize = map_b.iter().map(|(_, v)| v / 2).sum();
    println!("count_a: {:?}", count_a);
    println!("count_b: {:?}", count_b);

    let ans = if count_a > count_b {
        "Takahashi"
    } else {
        "Aoki"
    };
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
