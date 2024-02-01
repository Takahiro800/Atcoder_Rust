#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        S: String,
        T: String
    };

    let S: Vec<i32> = S.chars().map(|c| c as i32 - 'A' as i32).collect();
    let T: Vec<i32> = T.chars().map(|c| c as i32 - 'A' as i32).collect();

    let len_s = S[0].abs_diff(S[1]);
    let len_t = T[0].abs_diff(T[1]);

    if len_s == len_t || len_s + len_t == 5 {
        println!("Yes");
    } else {
        println!("No");
    }
}
