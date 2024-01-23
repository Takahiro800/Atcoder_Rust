#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        mut WX: [(usize,usize);N]
    };

    const m: usize = 24;
    let mut cnt = [0; m];

    for (w, x) in WX {
        for j in 9..18 {
            cnt[(j + x) % m] += w;
        }
    }
    let ans = cnt.iter().max().unwrap();
    println!("{}", ans);
}
