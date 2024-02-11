#![allow(non_snake_case)]
use std::collections::HashMap;

use proconio::input;

fn recursive(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if n == 2 {
        return n;
    } else if n == 3 {
        return 5;
    }

    if let Some(&result) = memo.get(&n) {
        return result;
    }
    let result = n + recursive(n / 2, memo) + recursive((n + 2 - 1) / 2, memo);
    memo.insert(n, result);
    result
}

fn main() {
    input! {
        N: usize
    };

    let mut memo = HashMap::new();
    let ans = recursive(N, &mut memo);
    println!("{}", ans);
}
