#![allow(non_snake_case)]
use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, Usize1);M],
    }

    let mut glaph: Vec<Vec<usize>> = vec![vec![]; N];
    let mut ans: Vec<_> = vec![-1; N];
    let mut queue: VecDeque<usize> = VecDeque::new();

    for (a, b) in &AB {
        glaph[*a].push(*b);
        glaph[*b].push(*a);
    }

    queue.push_back(0);
    ans[0] = 0;

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        for &to in &glaph[current] {
            if ans[to] == -1 {
                ans[to] = ans[current] + 1;
                queue.push_back(to);
            }
        }
    }

    for a in ans.iter() {
        println!("{}", a)
    }
}
