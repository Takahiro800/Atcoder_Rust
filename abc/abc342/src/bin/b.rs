#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        P: [usize; N],
        Q: usize,
        AB: [(usize, usize);Q]
    };

    for (a, b) in AB {
        let p = P.iter().position(|p| p == &a);
        let q = P.iter().position(|p| p == &b);

        if p < q {
            println!("{}", a)
        } else {
            println!("{}", b)
        }
    }
}
