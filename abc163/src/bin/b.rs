use proconio::input;

fn main() {
    input! {
      n: i32,
      m: i32,
      a: [i32; m],
    };

    let sum_a = a.iter().sum::<i32>();
    if n >= sum_a {
        println!("{}", n - sum_a);
    } else {
        println!("-1");
    }
}
