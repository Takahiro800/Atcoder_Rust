#![allow(non_snake_case)]
use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        N: usize,
        Q: usize,
    };

    let mut record = VecDeque::with_capacity(N);

    for i in 1..=N {
        record.push_back((i as i64, 0))
    }

    for _ in 0..Q {
        input! { num: usize }

        match num {
            1 => {
                input! { direction: char }
                move_head(&mut record, direction)
            }
            2 => {
                input! { p: usize }
                print_point(&record, p)
            }
            _ => unreachable!(),
        }
    }
}

fn move_head(record: &mut VecDeque<(i64, i64)>, q: char) {
    record.pop_back();

    let head = record.front().unwrap_or(&(0, 0));
    match q {
        'R' => {
            record.push_front((head.0 + 1, head.1));
        }
        'L' => {
            record.push_front((head.0 - 1, head.1));
        }
        'U' => {
            record.push_front((head.0, head.1 + 1));
        }
        'D' => {
            record.push_front((head.0, head.1 - 1));
        }
        _ => unreachable!(),
    };
}

fn print_point(record: &VecDeque<(i64, i64)>, q: usize) {
    if let Some(&ans) = record.get(q - 1) {
        println!("{} {}", ans.0, ans.1);
    } else {
        panic!("Invalid index");
    }
}
