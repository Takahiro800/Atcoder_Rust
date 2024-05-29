#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: u128,
        L: u128,
    };
    const MOD: u128 = 10_u128.pow(9) + 7;

    let mut ans = 0;

    for i in 0..=N / L {
        let n = N - (L * i) + i;
        ans += combination(n, i) % MOD;
    }

    println!("{}", ans);
}

fn combination(n: u128, r: u128) -> u128 {
    let mut result = 1;

    for i in 0..r {
        result = result * (n - i) / (i + 1);
    }
    result
}
