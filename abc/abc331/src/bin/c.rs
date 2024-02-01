#![allow(non_snake_case)]
use itertools::*;
use proconio::input;
use superslice::*;

fn main() {
    input! {
        N: usize,
        A: [usize;N]
    };

    let mut z = A.clone();
    z.sort();

    let mut sum = z.clone();
    sum.push(0);

    for i in (0..N).rev() {
        sum[i] += sum[i + 1];
    }

    println!(
        "{}",
        A.iter()
            .map(|a| {
                let x = z.upper_bound(a);
                sum[x]
            })
            .join(" ")
    );
}
