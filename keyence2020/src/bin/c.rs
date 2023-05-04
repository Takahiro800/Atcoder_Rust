use proconio::input;

fn main() {
    input! {
        n: u64,
        k: u64,
        s: u64,
    };

    let ans: Vec<u64> = if s < 10u64.pow(9) {
        vec![s; k as usize]
            .into_iter()
            .chain(vec![s + 1; (n - k) as usize].into_iter())
            .collect()
    } else {
        vec![s; k as usize]
            .into_iter()
            .chain(vec![1; (n - k) as usize].into_iter())
            .collect()
    };

    let result: String = ans
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", result);
}
