#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        S: String,
        T: String
    };

    let ans = if S == "AtCoder" && T == "Land" {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
