#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        XY: [(usize, usize); N]
    };

    let mut X = 0;
    let mut Y = 0;

    for (x, y) in XY {
        X += x;
        Y += y;
    }

    match X.cmp(&Y) {
        std::cmp::Ordering::Greater => println!("Takahashi"),
        std::cmp::Ordering::Less => println!("Aoki"),
        std::cmp::Ordering::Equal => println!("Draw"),
    }
}
