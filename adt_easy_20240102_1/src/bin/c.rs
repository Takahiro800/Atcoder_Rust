#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        N: usize,
        S: [usize; N]
    };

    let mut nums: Vec<usize> = Vec::new();
    for i in 1..=143 {
        for j in i..=143 {
            let num = 4 * i * j + 3 * i + 3 * j;
            if num <= 1000 {
                nums.push(num);
            }
        }
    }

    let filtered_nums: Vec<&usize> = S.iter().filter(|s| !nums.contains(s)).collect();
    println!("{}", filtered_nums.len())
}
