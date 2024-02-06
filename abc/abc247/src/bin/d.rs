#![allow(non_snake_case)]
use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        Q: usize,
    };

    let mut que = VecDeque::new();
    for _ in 0..Q {
        input! {q: usize}

        match q {
            1 => {
                input! { x: usize, c: usize };
                que.push_back((c, x))
            }
            2 => {
                input! { mut count: usize };
                let mut sum = 0;
                while count > 0 {
                    if let Some((mut c, x)) = que.pop_front() {
                        match count.cmp(&c) {
                            std::cmp::Ordering::Less => {
                                sum += count * x;
                                c -= count;
                                count = 0;
                                que.push_front((c, x))
                            }
                            _ => {
                                sum += c * x;
                                count -= c;
                            }
                        }
                    }
                }
                println!("{}", sum);
            }
            _ => unreachable!(),
        }
    }
}
