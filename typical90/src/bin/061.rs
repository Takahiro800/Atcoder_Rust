#![allow(non_snake_case)]
use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        Q: usize,
    };

    let mut set = VecDeque::new();

    for _ in 0..Q {
        input! {
            t: usize,
            x: usize
        }

        match t {
            1 => set.push_front(x),
            2 => set.push_back(x),
            3 => println!("{}", set.get(x - 1).unwrap()),
            _ => unreachable!(),
        }
    }
}
