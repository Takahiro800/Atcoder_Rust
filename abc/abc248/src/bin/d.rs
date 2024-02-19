#![allow(non_snake_case)]
use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        N: usize,
        A: [usize;N],
        Q: usize,
        Query: [(Usize1, Usize1, usize);Q]
    };

    let mut a = A
        .iter()
        .enumerate()
        .map(|(i, a)| (a, i))
        .collect::<Vec<_>>();
    a.sort();

    for (l, r, x) in Query {
        let ans = a.upper_bound(&(&x, r)) - a.lower_bound(&(&x, l));
        println!("{}", ans);
    }
}
