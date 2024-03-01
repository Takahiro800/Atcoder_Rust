#![allow(non_snake_case)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        mut S: Chars,
        Q: usize,
        CD: [(char, char); Q]
    };

    let mut replace_to = (b'a'..=b'z').map(|c| c as char).collect::<Vec<_>>();

    for (c, d) in CD {
        for i in replace_to.iter_mut() {
            if *i == c {
                *i = d;
            }
        }
    }

    for i in 0..N {
        S[i] = replace_to[(S[i] as u8 - b'a') as usize];
    }

    let ans: String = S.into_iter().collect();
    println!("{}", ans);
}
