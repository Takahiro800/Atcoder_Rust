#![allow(non_snake_case)]
use proconio::{input, marker::Chars};

fn main() {
    input! {
        N: usize,
        M: usize,
        S: [Chars; N]
    };

    let mut ans = M;
    for bit in 0..(1 << N) {
        let bit = bit as usize;
        let mut ok = true;
        for j in 0..M {
            let mut found = false;
            for i in 0..N {
                if S[i][j] == 'o' && (bit & (1 << i)) != 0 {
                    found = true;
                    break;
                }
            }
            if !found {
                ok = false;
                break;
            }
        }
        if ok {
            ans = ans.min(bit.count_ones() as usize);
        }
    }

    println!("{}", ans);
}
