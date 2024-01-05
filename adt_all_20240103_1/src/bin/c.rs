#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        P: usize,
        a: [usize; N]
    };

    let ans = a.iter().filter(|&a| a < &P).count();
    println!("{}", ans)
}
