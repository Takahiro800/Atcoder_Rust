#![allow(non_snake_case)]
use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        N: usize,
        X: usize,
        AB: [(usize,usize); N]
    };

    let mut set = HashSet::new();
    set.insert(0);

    for (a, b) in AB {
        let mut next = HashSet::new();

        for x in &set {
            next.insert(x + a);
            next.insert(x + b);
        }

        set = next;
    }

    println!("{}", if set.contains(&X) { "Yes" } else { "No" });
}
