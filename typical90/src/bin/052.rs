#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [[usize;6]; N]
    };

    const MOD: usize = 10_usize.pow(9) + 7;
    let ans = A.iter().fold(1, |s, a| s * a.iter().sum::<usize>() % MOD);

    println!("{}", ans % MOD);
}
