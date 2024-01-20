#![allow(non_snake_case)]
use std::collections::{HashMap, VecDeque};

use itertools::*;
use proconio::input;
use proconio::marker::Chars;
use superslice::*;

fn main() {
    input! {
        H: usize,
        W: usize,
        K: usize,
        S: [Chars;H]
    };

    let mut current = 0;
    for s in S {
        let set = run_length_encoding(s);
        let mut stack = VecDeque::new();
        // o, .とする
        let mut map = [0, 0];

        for (c, k) in set {
            match c {
                'o' => {
                    stack.push_back((c, k));
                    map[0] += k;
                    if map.iter().sum() >= K {
                        let diff = K - map[0];
                        if diff <= 0 {
                            println!("0");
                            return;
                        } else {
                            current = current.min(diff);
                        }
                    }
                }
            }
        }
    }
}

fn run_length_encoding<T: Eq>(a: Vec<T>) -> Vec<(T, usize)> {
    let mut a = a.into_iter().map(|a| (a, 1)).collect::<Vec<_>>();
    a.dedup_by(|a, b| {
        a.0 == b.0 && {
            b.1 += a.1;
            true
        }
    });

    a
}
