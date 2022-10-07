use proconio::input;

fn main() {
    input! {
      a: i32,
      b: i32
    }
    for element in a..=b {
        if 100 % element == 0 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
