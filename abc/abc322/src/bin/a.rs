#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        _N: usize,
        S: String
    };

    let ans = S.find("ABC").map_or(-1, |i| i as isize + 1);
    println!("{}", ans);
}
