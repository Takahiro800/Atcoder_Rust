use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
        n: usize,
        s: usize,
        A: [usize; n]
    }

    let mut dp: Vec<Vec<bool>> = vec![vec![false; s + 1]; n + 1];

    dp[0][0] = true;

    for i in 1..=n {
        for j in 0..=s {
            if dp[i - 1][j] == false {
                continue;
            }

            dp[i][j] = true;
            if j + A[i - 1] <= s {
                dp[i][j + A[i - 1]] = true;
            }
        }
    }

    if dp[n][s] == true {
        println!("Yes");
        return;
    }

    println!("No");
}
