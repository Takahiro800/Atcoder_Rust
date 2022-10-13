use proconio::input;

fn main() {
    input! {
      d: i32,
      n: i32,
      t: [(i32, i32); n]
    };

    let mut count: Vec<i32> = vec![0; (d + 1) as usize];

    for (start, end) in t {
        count[start as usize] += 1;

        if end < d {
            count[(end + 1) as usize] -= 1;
        }
    }

    let mut sum: i32 = 0;
    for i in 1..=d {
        sum += count[i as usize];
        println!("{}", sum);
    }
}
