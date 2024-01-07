#![allow(non_snake_case)]
use proconio::input;

fn main() {
    input! {
        n: usize
    };

    let mut ans: Vec<Vec<usize>> = vec![vec![0; n]; n];
    let mut p = 1;

    for k in 0..(n - 1) >> 1 {
        for i in 0..n - 1 - k * 2 {
            ans[k][i + k] = p;
            p += 1;
        }

        for i in 0..n - 1 - k * 2 {
            ans[i + k][n - 1 - k] = p;
            p += 1;
        }

        for i in 0..n - 1 - k * 2 {
            ans[n - 1 - k][n - i - 1 - k] = p;
            p += 1;
        }

        for i in 0..n - 1 - k * 2 {
            ans[n - i - 1 - k][k] = p;
            p += 1;
        }
    }

    let ans = &ans;
    let ans = (0..n)
        .map(|i| {
            (0..n)
                .map(|j| {
                    if (i, j) == ((n - 1) >> 1, (n - 1) >> 1) {
                        "T".to_string()
                    } else {
                        ans[i][j].to_string()
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    print_gird(ans)
}

fn print_gird(grid: Vec<Vec<String>>) {
    for line in grid {
        print_line(&line)
    }
}

fn print_line(line: &[String]) {
    let line_str: Vec<String> = line.iter().map(|x| x.to_string()).collect();
    let result = line_str.join(" ");
    println!("{}", result);
}
