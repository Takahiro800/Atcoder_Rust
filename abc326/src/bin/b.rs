#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
       mut n: usize
    };

    while n < 1000 {
        let a = n / 100;
        let b = (n / 10) % 10;
        let c = n % 10;

        if a * b == c {
            println!("{}", n);
            return;
        }
        n += 1;
    }
}
