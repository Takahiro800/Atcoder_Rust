#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
       mut n: usize,
    };

    let mut a = vec![];
    while n > 0 {
        a.push((n % 26) as u8);
        n /= 26;
    }

    let mut b = vec![];
    for a in a.iter().rev() {
        b.push((a + b'a' - 1) as char);
    }
    let ans = b.iter().join("");
    println!("{}", ans);
}
