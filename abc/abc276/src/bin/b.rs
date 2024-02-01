use proconio::input;
use std::collections::BTreeSet;

#[allow(non_snake_case)]
fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [[usize; 2]; M]
    };

    let mut town = vec![BTreeSet::new(); N];

    for e in AB.iter() {
        town[e[0] - 1].insert(e[1]);
        town[e[1] - 1].insert(e[0]);
    }

    for e in town.iter_mut() {
        print!("{}", e.len());
        for j in e.iter() {
            print!(" {}", j);
        }
        print!("\n");
    }
}
