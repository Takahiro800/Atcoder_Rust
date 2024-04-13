#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [isize; N-1]
    };

    let sum = A.iter().sum::<isize>();
    let ans = -sum;
    println!("{}", ans);
}
