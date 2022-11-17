use proconio::input;
use proconio::marker::Chars;

#[allow(non_snake_case)]
fn main() {
    input! {
        S: Chars,
    };

    let n = S.len();
    for i in (0..n).rev() {
        if S[i] == 'a' {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
