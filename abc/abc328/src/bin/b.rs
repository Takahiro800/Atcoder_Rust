#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        D: [usize;N]
    };

    let mut ans = 0;
    for n in 1..=N {
        let d = D[n - 1];
        for d in 1..=d {
            if is_all_same_digits(&n) && is_all_same_digits(&d) && n % 10 == d % 10 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

fn is_all_same_digits(n: &usize) -> bool {
    let s = n.to_string();
    s.chars().all(|c| c == s.chars().next().unwrap())
}
