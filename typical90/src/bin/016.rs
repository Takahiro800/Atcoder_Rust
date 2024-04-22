#![allow(non_snake_case)]
use itertools::iproduct;
use proconio::input;

fn main() {
    input! {
        N: usize,
        a: usize,
        b: usize,
        c: usize
    };

    let mut ans = std::usize::MAX;

    for (i, j) in iproduct!(0..10000, 0..10000) {
        let sum = a * i + b * j;

        if sum <= N && (N - sum) % c == 0 {
            let k = (N - sum) / c;
            ans = ans.min(i + j + k);
        }
    }

    println!("{}", ans);
}
