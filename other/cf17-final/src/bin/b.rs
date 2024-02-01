use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        s: String,
    };

    let mut char_count: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }

    let min_v = *char_count.values().min().unwrap();
    let max_v = *char_count.values().max().unwrap();

    if s.len() == 1 {
        println!("{}", "YES");
        return;
    }

    if s.len() == 2 {
        if max_v == 2 {
            println!("{}", "NO");
        } else {
            println!("{}", "YES");
        }
        return;
    }

    if s.len() == 3 {
        if max_v == 1 {
            println!("{}", "YES");
        } else {
            println!("{}", "NO");
        }
        return;
    }

    let diff = max_v - min_v;

    if diff > 1 || char_count.len() < 3 {
        println!("{}", "NO");
    } else {
        println!("{}", "YES");
    }
}
