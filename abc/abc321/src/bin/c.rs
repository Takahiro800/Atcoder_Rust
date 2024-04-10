#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        K: usize,
    };

    let numbers = (2..1 << 10)
        .map(|bs| {
            (0..10)
                .rev()
                .filter(|i| bs >> i & 1 == 1)
                .fold(0_u64, |acc, x| acc * 10 + x)
        })
        .sorted()
        .collect_vec();

    println!("{}", numbers[K - 1])
}
