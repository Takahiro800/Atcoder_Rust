use proconio::input;

fn main() {
    input! {
      n: usize,
      s_vec: [usize; n-1],
    }
    let mut counter = vec![0; n];

    for s in s_vec.iter() {
        counter[s - 1] += 1;
    }

    for m in counter {
        println!("{}", m);
    }
}
