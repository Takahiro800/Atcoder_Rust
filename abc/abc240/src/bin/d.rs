#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        A: [usize;N]
    };

    let mut stack = Vec::<(usize, usize)>::new();
    for &x in A.iter() {
        let count = if stack.last().map(|&(y, _)| y) == Some(x) {
            stack.last().unwrap().1 + 1
        } else {
            1
        };
        stack.push((x, count));

        while stack.last().filter(|&(x, count)| x == count).is_some() {
            let count = stack.last().unwrap().1;
            for _ in 0..count {
                stack.pop().unwrap();
            }
        }

        let ans = stack.len();
        println!("{}", ans);
    }
}
