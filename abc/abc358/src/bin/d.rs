#![allow(non_snake_case)]
use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize;N],
        mut B: [usize;M],
    };

    let mut map: BTreeMap<usize, usize> = BTreeMap::new();

    for &a in A.iter() {
        *map.entry(a).or_insert(0) += 1;
    }

    let mut ans: isize = 0;
    for &b in B.iter() {
        let mut i = map.range(b..);
        let min = i.next();

        match min {
            Some((&v, &count)) => {
                ans += v as isize;
                if count > 1 {
                    map.insert(v, count - 1);
                } else {
                    map.remove(&v);
                }
            }
            None => {
                ans = -1;
                break;
            }
        }
    }
    println!("{}", ans);
}
