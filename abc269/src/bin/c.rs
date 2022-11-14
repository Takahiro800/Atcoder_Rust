use proconio::input;

fn main() {
    input! {
      x: usize,
    }

    let mut ans = vec![0];
    let mut y = x;

    while y > 0 {
        ans.push(y);
        y = (y - 1) & x;
    }

    ans.sort();
    for a in ans {
        println!("{}", a);
    }
}
