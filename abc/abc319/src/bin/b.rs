#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize
    };

    let mut ans = vec![];

    for i in 0..=N {
        if i == 0 {
            ans.push('1');
        } else {
            let j = (1..=9)
                .filter(|j| N % j == 0 && i % (N / j) == 0)
                .min()
                .map_or('-', |j| (j as u8 + b'0') as char);
            ans.push(j);
        }
    }
    println!("{}", ans.iter().join(""));
}
