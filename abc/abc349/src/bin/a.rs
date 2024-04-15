#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [isize; N-1]
    };

    let ans = -A.iter().sum::<isize>();
    println!("{}", ans);
}
