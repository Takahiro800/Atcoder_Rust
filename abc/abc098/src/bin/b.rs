use std::collections::HashSet;

use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        s: String,
    };

    let mut ans = 0;

    for i in 1..n {
        let left = s[0..i].chars().collect::<HashSet<char>>();
        let right = s[i..].chars().collect::<HashSet<char>>();

        let intersection_count = left.iter().filter(|c| right.contains(c)).count();

        ans = ans.max(intersection_count)
    }
    println!("{}", ans);
}
