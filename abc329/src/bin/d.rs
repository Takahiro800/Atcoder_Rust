#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        A: [usize;M]
    };

    let mut count = vec![0; N + 1];
    let mut ans = (0, 0);
    for a in A {
        count[a] += 1;
        ans = ans.max((count[a], !a));
        println!("{}", !ans.1)
    }
}
