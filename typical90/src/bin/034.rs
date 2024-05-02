#![allow(non_snake_case)]
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        N: usize,
        K: usize,
        A: [usize;N]
    };

    let mut j = 0;
    let mut ans = 0;
    let mut map = HashMap::new();

    for i in 0..N {
        while j != N && (map.len() < K || map.contains_key(&A[j])) {
            *map.entry(&A[j]).or_insert(0) += 1;
            j += 1;
        }
        ans = ans.max(j - i);

        if map[&A[i]] == 1 {
            map.remove(&A[i]);
        } else {
            *map.get_mut(&A[i]).unwrap() -= 1;
        }
    }

    println!("{}", ans);
}
