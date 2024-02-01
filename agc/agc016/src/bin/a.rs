use proconio::input;
// use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    input! {
        s: String,
    };

    let chars: Vec<char> = s.chars().collect();
    let mut ans = chars.len();

    for c in b'a'..=b'z' {
        let c: char = std::char::from_u32(c as u32).unwrap();
        let mut count = 0;
        let mut chars: Vec<char> = s.chars().collect();

        loop {
            let set: HashSet<char> = chars.iter().cloned().collect();
            if set.len() == 1 {
                break;
            }

            let mut t: Vec<char> = Vec::with_capacity(chars.len() - 1);
            for i in 0..chars.len() - 1 {
                if chars[i] == c {
                    t.push(chars[i])
                } else {
                    t.push(chars[i + 1])
                }
            }

            count += 1;
            chars = t;
        }

        if count < ans {
            ans = count;
        }
    }
    println!("{}", ans);
}
