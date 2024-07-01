#![allow(non_snake_case)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut S: Chars,
        T: Chars,
    };

    let t_len = T.len();
    let s_len = S.len();

    if t_len == 1 {
        let first_char = T.first().unwrap();
        let ans = S[..S.len() - 1].contains(first_char);
        println!("{}", if ans { "Yes" } else { "No" });
        return;
    }

    let chunk_len = s_len / t_len;

    // for p in chunk_len - 1..=100 {
    for p in 1..=100 {
        if p > 0 {
            let chunks: Vec<_> = S.chunks(p).collect();

            for i in 0..chunk_len {
                let transformed: Vec<_> = chunks
                    .iter()
                    .filter_map(|chunk| chunk.get(i))
                    .cloned()
                    .collect();

                if transformed == T {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
