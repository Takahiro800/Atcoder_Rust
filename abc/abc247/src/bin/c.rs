#![allow(non_snake_case)]
use itertools::*;
use proconio::input;
// use superslice::*;

fn main() {
    input! {N :usize};

    let mut s = Vec::new();
    s.push(1);

    for i in 1..N {
        let mut temp = s.clone();
        s.push(i + 1);
        s.append(&mut temp);
    }

    println!("{}", s.iter().join(" "));
}
