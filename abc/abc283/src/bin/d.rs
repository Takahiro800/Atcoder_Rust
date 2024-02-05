#![allow(non_snake_case)]
use proconio::{input, marker::Bytes};
use std::collections::HashSet;

fn main() {
    input! {
        S: Bytes
    };
    let mut x = HashSet::new();
    let mut y: Vec<Vec<u8>> = vec![];

    for s in S {
        match s {
            b'(' => y.push(vec![]),
            b')' => {
                if let Some(top) = y.last() {
                    for y in top {
                        x.remove(y);
                    }
                    y.pop();
                }
            }
            _ => {
                if x.insert(s) {
                    if let Some(top) = y.last_mut() {
                        top.push(s);
                    }
                } else {
                    println!("No");
                    return;
                }
            }
        }
    }

    println!("Yes");
}
