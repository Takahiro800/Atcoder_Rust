#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        S: [String;3],
        T: String
    };

    let mut ans = String::new();
    for i in T.chars() {
        let index = i.to_digit(10).unwrap() as usize - 1;
        ans.push_str(&S[index]);
    }

    println!("{}", ans)
}
