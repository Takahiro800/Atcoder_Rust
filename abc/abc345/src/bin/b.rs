#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        X: isize
    };

    let mut ans;
    if X >= 0 {
        ans = (X + 10 - 1) / 10;
    } else {
        if X % 10 == 0 {
            ans = X / 10;
        } else {
            ans = X / 10;
        }
    }
    println!("{}", ans);
}
