#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        K: isize,
        mut A: [isize;N]
    };

    A.sort();

    if K > 0 || (A[0] >= 0) {
        println!("Yes");
        println!("{}", A.iter().join(" "));
    } else if A.iter().map(|&a| a).sum::<isize>() >= K {
        A.reverse();
        println!("Yes");
        println!("{}", A.iter().join(" "));
    } else {
        println!("No");
    };
}
