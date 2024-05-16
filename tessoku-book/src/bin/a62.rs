#![allow(non_snake_case)]
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(Usize1, Usize1); M],
    }
    let mut graph = vec![vec![]; N];
    let mut visited = vec![false; N];

    for &(a, b) in AB.iter() {
        graph[a].push(b);
        graph[b].push(a);
    }

    dfs(&graph, &mut visited, 0);

    let ans = if visited.iter().all(|&a| a) {
        "The graph is connected."
    } else {
        "The graph is not connected."
    };

    println!("{}", ans);
}

fn dfs(graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, pos: usize) {
    visited[pos] = true;

    for &node in &graph[pos] {
        if !visited[node] {
            dfs(graph, visited, node);
        }
    }
}
