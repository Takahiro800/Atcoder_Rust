#![allow(non_snake_case)]
use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize;N],
        M: usize,
        B: [usize;M],
        L: usize,
        C: [usize;L],
        Q: usize,
        X: [usize;Q]
    };

    let mut set = BTreeSet::new();

    for a in A.iter() {
        for b in B.iter() {
            for c in C.iter() {
                let sum = a + b + c;
                set.insert(sum);
            }
        }
    }

    for x in X.iter() {
        if set.contains(x) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
