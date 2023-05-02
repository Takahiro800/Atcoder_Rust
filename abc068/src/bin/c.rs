use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [[usize; 2]; m]
    };

    let mut a: Vec<usize> = vec![];
    let mut b: HashSet<usize> = HashSet::new();

    for i in &mut ab {
        i.sort();
        if i[0] == 1 {
            a.push(i[1]);
        }
        if i[1] == n {
            b.insert(i[0]);
        }
    }

    let common: Vec<usize> = a.into_iter().filter(|p| b.contains(p)).collect();

    if common.len() > 0 {
        println!("{}", "POSSIBLE");
    } else {
        println!("{}", "IMPOSSIBLE");
    }
}
