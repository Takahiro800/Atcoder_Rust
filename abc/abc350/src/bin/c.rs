#![allow(non_snake_case)]
use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        N: usize,
        mut A: [usize; N]
    };

    let mut B = A.clone();
    B.sort();
    let mut ans = vec![];
    let mut map: HashMap<usize, usize> = A.iter().enumerate().map(|(i, &a)| (a, i)).collect();

    for i in 0..N {
        if A[i] != B[i] {
            let j = map[&B[i]];
            A.swap(i, j);
            map.insert(A[i], i);
            map.insert(A[j], j);
            ans.push((i + 1, j + 1));
        }
    }

    println!("{}", ans.len());
    for &(a, b) in ans.iter() {
        println!("{} {}", a, b);
    }
}
