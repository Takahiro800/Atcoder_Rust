use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
      t: usize,
      n: i32,
      x: [(usize, usize); n]
    }

    let mut count_array = vec![0; t + 1];

    for (start, end) in x {
        count_array[start] += 1;
        count_array[end] -= 1;
    }

    let mut count = 0;
    for i in 0..t {
        count += count_array[i];
        println!("{}", count);
    }
}
