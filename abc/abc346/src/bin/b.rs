#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        W: usize,
        B: usize
    };

    let s = "wbwbwwbwbwbw";
    let S = s.repeat(20);

    for i in 0..12 {
        let w = S[i..(i + W + B)].chars().filter(|c| *c == 'w').count();
        let b = S[i..(i + W + B)].chars().filter(|c| *c == 'b').count();

        if w == W && b == B {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
