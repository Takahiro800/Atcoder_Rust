#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        XY: [(usize, usize); N]
    };

    let x: usize = XY.iter().map(|(x, _)| x).sum();
    let y: usize = XY.iter().map(|(_, y)| y).sum();

    match x.cmp(&y) {
        std::cmp::Ordering::Greater => println!("Takahashi"),
        std::cmp::Ordering::Equal => println!("Draw"),
        std::cmp::Ordering::Less => println!("Aoki"),
    }
}
