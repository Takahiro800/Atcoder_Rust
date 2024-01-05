#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        XY: f32
    };

    let x = XY.floor() as usize;
    let y = (XY * 10.0) as usize % 10;

    match y {
        0..=2 => println!("{}", format!("{}-", x)),
        3..=6 => println!("{}", x),
        7..=9 => println!("{}", format!("{}+", x)),
        _ => unreachable!(),
    }
}
