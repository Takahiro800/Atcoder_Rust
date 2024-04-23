#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        E: [(usize, usize);N-1]
    };

    let mut graph = vec![vec![]; N + 1];
    for &(a, b) in E.iter() {
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut ans = vec![vec![]; 2];
    let mut stack = vec![(1, 0, 0)];
    while let Some((v, p, d)) = stack.pop() {
        ans[d].push(v);

        for &u in graph[v].iter() {
            if u != p {
                stack.push((u, v, d ^ 1));
            }
        }
    }

    if ans[0].len() < ans[1].len() {
        ans.swap(0, 1);
    }
    println!("{}", ans[0].iter().take(N / 2).join(" "));
}
