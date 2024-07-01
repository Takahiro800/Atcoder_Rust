#![allow(non_snake_case)]
use std::vec;

use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        N: usize,
        T: isize,
        S: Chars,
        X: [isize; N]
    };

    let mut rights = vec![];
    let mut lefts = vec![];

    for (d, x) in S.iter().zip(X.iter()) {
        if d == &'0' {
            lefts.push(x);
        } else {
            rights.push(x);
        }
    }

    lefts.sort();
    rights.sort();

    let mut ans = 0;
    let diff = 2 * T;

    for r in rights.iter() {
        let upper = *r + diff + 1;
        let lower = *r;

        let up_count = lefts.lower_bound(&&upper);
        let lower_count = lefts.upper_bound(&&lower);

        ans += up_count - lower_count;
    }

    println!("{}", ans);
}
