use proconio::input;

fn main() {
    input! {
      n: i32,
      x: i32,
      a: [i32; n]
    }

    for element in a {
        if element == x {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
