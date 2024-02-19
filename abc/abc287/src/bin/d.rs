#![allow(non_snake_case)]
use proconio::{input, marker::Bytes};

fn main() {
    input! {
        mut S: Bytes,
        mut T: Bytes,
    };

    let mut ans = vec![];
    for _ in 0..2 {
        let cnt = S
            .iter()
            .zip(T.iter())
            .take_while(|(&s, &t)| s == t || s == b'?' || t == b'?')
            .count();
        ans.push(cnt);

        S.reverse();
        T.reverse();
    }

    for x in 0..=T.len() {
        println!(
            "{}",
            if x <= ans[0] && T.len() - x <= ans[1] {
                "Yes"
            } else {
                "No"
            }
        );
    }
}
