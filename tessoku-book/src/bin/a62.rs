#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [(usize, usize);M]
    }
    let mut glaph: Vec<Vec<usize>> = vec![vec![]; N + 1];
    let mut visited: Vec<_> = vec![false; N + 1];
    visited[0] = true;

    for (a, b) in &AB {
        glaph[*a].push(*b);
        glaph[*b].push(*a);
    }

    dfs(&glaph, &mut visited, 1);

    let ans: &str = if visited.iter().all(|&b| b) {
        "The graph is connected."
    } else {
        "The graph is not connected."
    };

    println!("{}", ans)
}

fn dfs(glaph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, pos: usize) {
    visited[pos] = true;

    for &node in &glaph[pos] {
        if !visited[node] {
            dfs(glaph, visited, node)
        }
    }
}
