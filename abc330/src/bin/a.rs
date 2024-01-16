#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        L: usize,
        A: [usize; N]
    };

    let ans = A.iter().filter(|&a| a >= &L).count();
    println!("{}", ans)
}
