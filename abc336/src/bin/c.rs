#![allow(non_snake_case)]
// use itertools::*;
use proconio::input;
// use superslice::*;

fn main() {
    input! {
        N: usize
    }

    println!("{}", good_number(N));
}

fn good_number(n: usize) -> usize {
    let mut n = n - 1;
    let mut result = 0;
    let mut place = 1;
    while n > 0 {
        result += (n % 5) * 2 * place;
        n /= 5;
        place *= 10;
    }
    result
}
