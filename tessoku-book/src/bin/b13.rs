use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    // 累積和の用意
    let mut cumulative_sum = vec![0; n + 1];
    for i in 1..=n {
        cumulative_sum[i] = cumulative_sum[i - 1] + a[i - 1];
    }

    // しゃくとり法
    let mut r: Vec<usize> = vec![0; n + 1];
    for i in 1..=n {
        if i == 1 {
            r[i] = 0;
        } else {
            r[i] = r[i - 1];
        }

        while r[i] < n && sum(&cumulative_sum, i, r[i] + 1) <= k {
            r[i] += 1;
        }
    }

    let mut ans: usize = 0;
    for i in 1..=n {
        ans += r[i] - i + 1;
    }

    println!("{}", ans);
}

fn sum(s: &Vec<usize>, l: usize, r: usize) -> usize {
    return s[r] - s[l - 1];
}
