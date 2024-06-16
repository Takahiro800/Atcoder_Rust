#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: usize,
        T: [usize; N]
    };

    let mut sum = 0;

    for (i, &t) in T.iter().enumerate() {
        if sum >= t {
            println!("{}", sum + A);
            sum += A;
        } else {
            println!("{}", t + A);
            sum = t + A;
        }
    }
}
