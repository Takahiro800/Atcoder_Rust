use proconio::input;
use std::cmp;

#[allow(non_snake_case)]
fn main() {
    input! {
        A: i32,
        B: i32,
    };
    let ans = cmp::max(A * B, cmp::max(A + B, A - B));

    println!("{}", ans);
}
