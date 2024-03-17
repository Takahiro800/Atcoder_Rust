#![allow(non_snake_case)]
use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        N: usize,
        mut A: [isize; N],
        Q: usize
    };

    let empty = std::isize::MAX;
    let start = -1;
    let tail = -2;
    let mut map = BTreeMap::new();

    A.insert(0, start);
    A.push(tail);

    for a in A.iter() {
        map.insert(*a, (empty, empty));
    }

    for a in A.windows(2) {
        map.get_mut(&a[0]).unwrap().1 = a[1];
        map.get_mut(&a[1]).unwrap().0 = a[0];
    }

    for _ in 0..Q {
        input! { op: usize }
        match op {
            1 => {
                input! {
                    x: isize,
                    y: isize
                };
                let r = map[&x].1;
                map.insert(y, (x, r));
                map.get_mut(&x).unwrap().1 = y;
                map.get_mut(&r).unwrap().0 = y;
            }
            2 => {
                input! { x: isize }

                let (l, r) = map.remove(&x).unwrap();
                map.get_mut(&l).unwrap().1 = r;
                map.get_mut(&r).unwrap().0 = l;
            }
            _ => unreachable!(),
        }
    }
    let mut ans = vec![];
    let mut pos = map[&start].1;
    while pos != tail {
        ans.push(pos);
        pos = map[&pos].1;
    }
    println!("{}", ans.iter().join(" "));
}
