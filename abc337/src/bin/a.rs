#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        XY: [(usize, usize); N]
    };

    let mut sum_x = 0;
    let mut sum_y = 0;

    for (x, y) in XY {
        sum_x += x;
        sum_y += y;
    }

    if sum_x > sum_y {
        println!("Takahashi");
    } else if sum_x < sum_y {
        println!("Aoki")
    } else {
        println!("Draw")
    }
}
