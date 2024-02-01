#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        _N: usize,
        Q: usize,
        S: String,
        LR: [(usize,usize);Q]
    };

    let mut cumlative_sum = vec![0];
    let chars: Vec<char> = S.chars().collect();
    let mut count = 0;

    for i in 0..chars.len() - 1 {
        if chars[i] == chars[i + 1] {
            count += 1;
        }
        cumlative_sum.push(count);
    }
    cumlative_sum.push(count);

    for (l, r) in LR {
        println!("{}", cumlative_sum[r - 1] - cumlative_sum[l - 1]);
    }
}
