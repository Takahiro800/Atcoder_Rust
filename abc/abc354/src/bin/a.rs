#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        H: usize,
    };

    let mut ans = 1;
    let mut height = 1;

    while height <= H {
        height += 2_usize.pow(ans);
        ans += 1;
    }

    println!("{}", ans);
}
