#![allow(non_snake_case)]
use std::{collections::HashMap, usize};

use proconio::input;

fn main() {
    input! {
        _N: usize,
        S: String
    };

    let mut hash_map: HashMap<char, usize> = HashMap::new();
    let mut current: Option<char> = None;
    let mut count = 0;

    for s in S.chars() {
        match current {
            Some(c) if c == s => {
                count += 1;
            }
            _ => {
                if let Some(c) = current {
                    match hash_map.get(&c) {
                        Some(value) => {
                            if count > *value {
                                hash_map.insert(c, count);
                            }
                        }
                        None => {
                            hash_map.insert(c, count);
                        }
                    };
                }
                current = Some(s);
                count = 1;
            }
        }
    }

    if let Some(c) = current {
        match hash_map.get(&c) {
            Some(value) => {
                if count > *value {
                    hash_map.insert(c, count);
                }
            }
            None => {
                hash_map.insert(c, count);
            }
        };
    }

    let ans = hash_map.values().sum::<usize>();
    println!("{}", ans)
}
