#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize
    };

    for i in 0..N {
        input! {
            A: [usize;N]
        }

        let mut line = vec![];
        for (i, a) in A.iter().enumerate() {
            if a == &1 {
                line.push(i + 1);
            }
        }
        println!("{}", line.iter().join(" "))
    }
}
