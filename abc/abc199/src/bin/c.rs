#![allow(non_snake_case)]
// use itertools::*;
use proconio::{input, marker::Bytes};
// use superslice::*;

fn main() {
    input! {
        N: usize,
        mut S: Bytes,
        Q: usize,
        Query: [(usize,usize,usize);Q]
    };

    let mut flip = false;

    for (t, mut a, mut b) in Query {
        match t {
            1 => {
                a -= 1;
                b -= 1;
                if flip {
                    a = (a + N) % (2 * N);
                    b = (b + N) % (2 * N);
                }
                S.swap(a, b)
            }
            2 => flip = !flip,
            _ => unreachable!(),
        }
    }
    if flip {
        S.rotate_left(N);
    }

    println!("{}", S.into_iter().map(|c| c as char).collect::<String>());
}
