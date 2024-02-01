use proconio::input;

fn main() {
    input! {
        n: usize
    };

    println!("{}", if 2 <= n && n <= 4 { "No" } else { "Yes" })
}
