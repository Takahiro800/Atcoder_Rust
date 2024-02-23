#![allow(non_snake_case)]
use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        L: usize,
        Q: usize,
        CX: [(usize, usize);Q]
    };
    let mut set = BTreeSet::new();
    set.insert(0);
    set.insert(L);

    for (c, x) in CX {
        if c == 1 {
            set.insert(x);
        } else {
            let l = set.range(..x).next_back().unwrap();
            let r = set.range(x..).next().unwrap();
            println!("{}", r - l);
        }
    }
}
