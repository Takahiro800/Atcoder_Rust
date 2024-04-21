#![allow(non_snake_case)]

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        CP: [(Usize1, usize);N],
        Q: usize,
        LR: [(usize, usize);Q]
    };

    let mut points = vec![(0, 0); N + 1];

    for (i, &(c, p)) in CP.iter().enumerate() {
        match c {
            0 => points[i + 1] = (points[i].0 + p, points[i].1),
            1 => points[i + 1] = (points[i].0, points[i].1 + p),
            _ => unreachable!(),
        }
    }

    for &(l, r) in LR.iter() {
        let p1 = points[r].0 - points[l - 1].0;
        let p2 = points[r].1 - points[l - 1].1;
        println!("{} {}", p1, p2);
    }
}
