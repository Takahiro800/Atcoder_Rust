use proconio::input;

fn main() {
    input! {
      h: usize,
      w: usize,
      n: usize,
      abcd: [(usize, usize, usize, usize); n]
    }

    let mut x = vec![vec![0; w + 2]; h + 2];

    for (a, b, c, d) in abcd {
        x[a][b] += 1;
        x[a][d + 1] -= 1;
        x[c + 1][b] -= 1;
        x[c + 1][d + 1] += 1;
    }

    let mut z = vec![vec![0; w + 1]; h + 1];

    // 二次元累積和をとる
    for i in 1..=h {
        for j in 1..=w {
            z[i][j] = z[i][j - 1] + x[i][j];
        }
    }

    for j in 1..=w {
        for i in 1..=h {
            z[i][j] += z[i - 1][j];
        }
    }

    for i in 1..=h {
        for j in 1..=w {
            print!("{}{}", z[i][j], if j == w { '\n' } else { ' ' });
        }
    }
}
