#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        R: f32,
        X: f32,
        Y: f32
    };

    let len = (X * X + Y * Y).sqrt();

    if len < R {
        println!("{}", 2);
    } else {
        println!("{}", (len / R).ceil());
    }
}
