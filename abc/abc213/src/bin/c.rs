#![allow(non_snake_case)]
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        _H: usize,
        _W: usize,
        N: usize,
        AB: [(Usize1,Usize1);N]
    };

    let mut arr_a: Vec<usize> = AB.iter().map(|ab| ab.0).collect();
    let mut arr_b = AB.iter().map(|ab| ab.1).collect::<Vec<_>>();
    arr_a.sort();
    arr_b.sort();
    arr_a.dedup();
    arr_b.dedup();

    for (a, b) in &AB {
        let x = arr_a.binary_search(a).unwrap();
        let y = arr_b.binary_search(b).unwrap();

        println!("{} {}", x + 1, y + 1);
    }
}
