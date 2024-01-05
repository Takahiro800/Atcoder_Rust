#![allow(non_snake_case)]
use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        N: usize,
        ST: [(String, String); N]
    };

    let mut hash_set: HashSet<(String, String)> = HashSet::new();

    for (s, t) in ST {
        if hash_set.contains(&(s.clone(), t.clone())) {
            println!("Yes");
            return;
        } else {
            hash_set.insert((s.clone(), t.clone()));
        }
    }
    println!("No")
}
