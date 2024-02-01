#![allow(non_snake_case)]
use proconio::input;

// 2進数表記にして、'1'を'2'に変換する
fn main() {
    input! {
        K: usize
    };

    let binary = format!("{:b}", K);
    let ans = binary.replace('1', "2");
    println!("{}", ans)
}
