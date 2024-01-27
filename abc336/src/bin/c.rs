#![allow(non_snake_case)]
// use itertools::*;
use proconio::input;
// use superslice::*;

fn main() {
    input! {
        N: usize
    }
    let mut n = N - 1;
    let mut ans = 0;
    let mut pow = 1;

    while n > 0 {
        ans += pow * (n % 5) * 2;
        n /= 5;
        pow *= 10;
    }

    println!("{}", ans);
}
