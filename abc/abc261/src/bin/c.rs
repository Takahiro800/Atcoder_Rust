#![allow(non_snake_case)]
use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        N: usize
    };

    let mut map = HashMap::new();

    for _ in 0..N {
        input! {
            S: String
        }

        match map.get(&S) {
            Some(c) => {
                println!("{}({})", S, c);
                map.insert(S, c + 1);
            }
            None => {
                println!("{}", S);
                map.insert(S, 1);
            }
        }
    }
}
