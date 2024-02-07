#![allow(non_snake_case)]
use proconio::{input, marker::Bytes};
use superslice::Ext;

fn main() {
    input! {
        N: usize,
        _M: usize,
        mut S: [Bytes;N]
    };

    S.sort();
    let mut ans = false;
    while {
        let mut ok = true;
        for s in S.windows(2) {
            let c = s[0]
                .iter()
                .zip(s[1].iter())
                .filter(|p| *p.0 != *p.1)
                .count();
            ok &= c == 1;
        }
        if ok {
            ans = true;
        };
        S.next_permutation()
    } {}
    println!("{}", if ans { "Yes" } else { "No" });
}
