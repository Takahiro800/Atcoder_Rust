#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize
    };

    let mut x = vec![];
    let mut a = 0usize;

    for _ in 0..12 {
        a *= 10;
        a += 1;
        x.push(a);
    }

    let mut res = vec![];

    for &a in &x {
        for &b in &x {
            for &c in &x {
                res.push(a + b + c);
            }
        }
    }
    res.sort_unstable();
    res.dedup();

    println!("{}", res[N - 1])
}
