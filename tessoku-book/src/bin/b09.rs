use proconio::input;

fn main() {
    input! {
      n: usize,
      abcd: [(usize, usize, usize, usize); n]
    }

    let max_length = 1500;
    let mut x = vec![vec![0; max_length + 2]; max_length + 2];

    for (a, b, c, d) in abcd {
        x[a + 1][b + 1] += 1;
        x[a + 1][d + 1] -= 1;
        x[c + 1][b + 1] -= 1;
        x[c + 1][d + 1] += 1;
    }

    let mut z = vec![vec![0; max_length + 1]; max_length + 1];

    for i in 1..=max_length {
        for j in 1..=max_length {
            z[i][j] = z[i][j - 1] + x[i][j];
        }
    }

    for j in 1..=max_length {
        for i in 1..=max_length {
            z[i][j] += z[i - 1][j];
        }
    }

    let mut count = 0;

    for row in z {
        count += row.iter().filter(|&e| e == &0).count();
    }
    let ans = (max_length + 1) * (max_length + 1) - count;
    println!("{}", ans);
}
