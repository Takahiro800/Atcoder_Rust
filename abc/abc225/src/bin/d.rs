#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        Q: usize,
    };

    let end: usize = N + 1;
    let mut state = vec![(0, end); N + 1];

    for _ in 0..Q {
        input! { num: usize };

        match num {
            1 => {
                input! {
                    x: usize,
                    y: usize,
                }
                state[x].1 = y;
                state[y].0 = x;
            }
            2 => {
                input! {
                    x: usize,
                    y: usize,
                }
                state[x].1 = end;
                state[y].0 = 0;
            }
            3 => {
                input! { x : usize }
                let mut ans = vec![];
                let mut pos = x;

                while state[pos].0 != 0 {
                    pos = state[pos].0;
                }
                while pos != end {
                    ans.push(pos);
                    pos = state[pos].1;
                }
                println!("{} {}", ans.len(), ans.iter().join(" "))
            }
            _ => unreachable!(),
        }
    }
}
