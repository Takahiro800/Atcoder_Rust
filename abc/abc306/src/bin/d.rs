#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        XY: [(usize, isize);N]
    };

    let mut a: isize = 0;
    let mut b: isize = 0;

    for &(x, y) in XY.iter() {
        match x {
            0 => a = a.max(a.max(b) + y),
            1 => b = b.max(a + y),
            _ => unreachable!(),
        }
    }
    let ans = a.max(b);
    println!("{}", ans);
}
