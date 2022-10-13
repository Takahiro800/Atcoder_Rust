use proconio::input;

#[allow(non_snake_case)]
fn main() {
    input! {
      N: usize,
      A: [usize; N],
      Q: usize,
      T: [(usize, usize); Q]
    };

    let mut hit_count_sum: Vec<usize> = vec![0];
    let mut lose_count_sum: Vec<usize> = vec![0];

    for i in 1..=N {
        if A[i - 1] == 1 {
            hit_count_sum.push(hit_count_sum[i - 1] + 1);
            lose_count_sum.push(lose_count_sum[i - 1]);
        } else {
            hit_count_sum.push(hit_count_sum[i - 1]);
            lose_count_sum.push(lose_count_sum[i - 1] + 1);
        }
    }

    for (start, end) in T {
        let hit_count = hit_count_sum[end] - hit_count_sum[start - 1];
        let lose_count = lose_count_sum[end] - lose_count_sum[start - 1];

        if hit_count == lose_count {
            println!("draw");
        } else if hit_count > lose_count {
            println!("win");
        } else {
            println!("lose");
        }
    }
}
