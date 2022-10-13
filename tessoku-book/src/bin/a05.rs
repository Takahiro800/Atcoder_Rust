use proconio::input;

fn main() {
    input! {
      n: i32,
      k: i32
    };

    let mut count: i32 = 0;

    for i in 1..=n {
        for j in 1..=n {
            let l: i32 = k - i - j;

            if l >= 1 && l <= n {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
