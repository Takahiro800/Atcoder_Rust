#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize;N]
    };

    let mut stack = vec![];

    for &a in A.iter() {
        // stack.push(2_usize.pow(a as u32));
        stack.push(a);
        let mut len = stack.len();

        while len >= 2 && stack[len - 1] == stack[len - 2] {
            let last = stack.pop().unwrap();
            stack.pop();
            let sum = last + 1;
            stack.push(sum);
            len = stack.len();
        }
    }

    let ans = stack.len();
    println!("{}", ans);
}
