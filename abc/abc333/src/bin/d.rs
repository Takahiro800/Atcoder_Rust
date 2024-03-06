#![allow(non_snake_case)]
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        UV: [(Usize1, Usize1); N-1]
    };

    let mut g = vec![vec![]; N];
    for (u, v) in UV {
        g[u].push(v);
        g[v].push(u);
    }

    let [ord, _parent] = sort_tree::sort_tree_remove_parent(0, &mut g);

    // 自身を含む子孫ノードの個数
    let mut sub_tree = vec![1; N];

    for &x in ord.iter().rev() {
        for &y in g[x].iter() {
            // 子ノードの個数を加算していく
            sub_tree[x] += sub_tree[y];
        }
    }

    let ans = N - g[0].iter().map(|&i| sub_tree[i]).max().unwrap();
    println!("{}", ans);
}

//---------- begin Sort Tree ----------
#[allow(dead_code)]
mod sort_tree {
    pub fn sort_tree_remove_parent(root: usize, g: &mut [Vec<usize>]) -> [Vec<usize>; 2] {
        let [ord, parent] = sort_tree(root, g);
        remove_parent(g, &parent);
        [ord, parent]
    }

    // 単一方向になるようにそれぞれの 隣接ノードから親ノードを削除
    pub fn remove_parent(g: &mut [Vec<usize>], parent: &[usize]) {
        g.iter_mut().enumerate().for_each(|(i, v)| {
            if let Some(i) = v.iter().position(|&y| y == parent[i]) {
                v.swap_remove(i);
            };
        });
    }

    pub fn sort_tree(root: usize, g: &[Vec<usize>]) -> [Vec<usize>; 2] {
        sort_tree_by(root, g, |x| *x)
    }

    pub fn sort_tree_by<E>(root: usize, g: &[Vec<E>], to: impl Fn(&E) -> usize) -> [Vec<usize>; 2] {
        let mut ord = Vec::new();
        let mut parent = vec![root; g.len()];
        sort_tree_impl(root, root, g, &to, &mut parent, &mut ord);
        [ord, parent]
    }

    pub fn sort_tree_impl<E>(
        x: usize,
        p: usize,
        g: &[Vec<E>],
        to: &impl Fn(&E) -> usize,
        parent: &mut [usize],
        ord: &mut Vec<usize>,
    ) {
        ord.push(x);
        parent[x] = p;
        g[x].iter()
            .map(to)
            .filter(|&y| y != p)
            .for_each(|y| sort_tree_impl(y, x, g, to, parent, ord))
    }
}
//---------- end Sort Tree ----------
