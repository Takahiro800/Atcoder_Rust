use proconio::input;

fn main() {
    input! {
      n: i32,
      q: i32,
      a: [i32; n],
      t: [(usize, usize); q]
    };

    // 累積和のset
    let mut cumulative_sum: Vec<i32> = vec![0];

    for i in 1..=n {
        cumulative_sum.push(cumulative_sum[(i - 1) as usize] + a[(i - 1) as usize]);
    }

    for (start, end) in t {
        println!("{}", cumulative_sum[end] - cumulative_sum[start - 1]);
    }
}
