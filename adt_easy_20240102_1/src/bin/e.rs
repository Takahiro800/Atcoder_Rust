#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        S: String
    };

    let mut chars: Vec<char> = S.chars().rev().collect();
    let mut ans = 0;

    while chars.len() > 1 {
        if chars[0] == '0' && chars[1] == '0' {
            chars = chars[2..].to_vec();
        } else {
            chars = chars[1..].to_vec();
        }
        ans += 1;
    }

    ans += 1;

    println!("{}", ans)
}
