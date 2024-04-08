#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        _N: usize,
        _M: usize,
        S: String,
        T: String
    };

    let ans = match (T.starts_with(&S), T.ends_with(&S)) {
        (true, true) => 0,
        (true, false) => 1,
        (false, true) => 2,
        (false, false) => 3,
    };

    println!("{}", ans);
}
