#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        _N: usize,
        S: String
    };

    let mut flag = false;
    let mut ans = String::new();

    for c in S.chars() {
        match c {
            '"' => {
                flag = !flag;
                ans.push('"');
            }
            ',' => {
                if flag {
                    ans.push(',');
                } else {
                    ans.push('.');
                }
            }
            _ => ans.push(c),
        }
    }

    println!("{}", ans)
}
