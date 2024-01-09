#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        _N: usize,
        K: usize,
        mut A: [usize;K]
    };

    match K % 2 {
        0 => {
            let chunks: Vec<_> = A.chunks(2).collect();
            let ans: usize = chunks.iter().map(|chunk| chunk[1] - chunk[0]).sum();

            println!("{}", ans)
        }
        1 => {
            if K == 1 {
                println!("0");
                return;
            }

            let pre_chunks = A
                .chunks(2)
                .filter(|chunk| chunk.len() == 2)
                .collect::<Vec<_>>();

            let pre_cumulative_sums = calc_cumulative_sums(pre_chunks);

            let suf_chunks = A
                .rchunks(2)
                .filter(|chunk| chunk.len() == 2)
                .collect::<Vec<_>>();

            let suf_cumulative_sums = calc_cumulative_sums(suf_chunks);

            let mut ans: usize = pre_cumulative_sums[K / 2 - 1];

            for i in 0..(K / 2) {
                if K as isize / 2 - 2 - i as isize >= 0 {
                    let sum = pre_cumulative_sums[i] + suf_cumulative_sums[K / 2 - 2 - i];
                    if ans > sum {
                        ans = sum;
                    }
                }
            }

            if let Some(&last) = suf_cumulative_sums.last() {
                if ans > last {
                    ans = last;
                }
            }

            println!("{}", ans)
        }
        _ => unreachable!(),
    }
}

fn calc_cumulative_sums(chunks: Vec<&[usize]>) -> Vec<usize> {
    let sums: Vec<_> = chunks.iter().map(|chunk| chunk[1] - chunk[0]).collect();

    let mut cumulative_sums = vec![0; sums.len()];
    cumulative_sums[0] = sums[0];

    for i in 1..sums.len() {
        cumulative_sums[i] = cumulative_sums[i - 1] + sums[i]
    }

    cumulative_sums
}

#[test]
fn test_chunk() {
    let A = [1, 2, 3, 4, 5, 6, 7];

    let pre = A
        .chunks(2)
        .filter(|chunk| chunk.len() == 2)
        .collect::<Vec<_>>();
    assert_eq!(pre, [[1, 2], [3, 4], [5, 6]]);

    let sums: Vec<_> = pre
        .iter()
        .map(|chunk| chunk.iter().sum::<usize>())
        .collect();
    assert_eq!(sums, [3, 7, 11]);

    let mut cumulative_sums = vec![0; sums.len()];
    cumulative_sums[0] = sums[0];
    for i in 1..sums.len() {
        cumulative_sums[i] = cumulative_sums[i - 1] + sums[i]
    }
    assert_eq!(cumulative_sums, [3, 10, 21])
}

#[test]
fn test_rchunk() {
    let A = [1, 2, 3, 4, 5, 6, 7];

    let suf = A
        .rchunks(2)
        .filter(|chunk| chunk.len() == 2)
        .collect::<Vec<_>>();
    assert_eq!(suf, [[6, 7], [4, 5], [2, 3]]);

    let sums: Vec<_> = suf
        .iter()
        .map(|chunk| chunk.iter().sum::<usize>())
        .collect();
    assert_eq!(sums, [13, 9, 5]);

    let mut cumulative_sums = vec![0; sums.len()];
    cumulative_sums[0] = sums[0];
    for i in 1..sums.len() {
        cumulative_sums[i] = cumulative_sums[i - 1] + sums[i]
    }
    assert_eq!(cumulative_sums, [13, 22, 27])
}

#[test]
fn test() {
    let A = [3, 10, 21];
    let B = [13, 22, 27];

    let n = 7;

    let mut ans: usize = A[2];

    for i in 0..(n / 2) - 1 {
        let sum = A[i] + B[n / 2 - 1 - i];
        if ans > sum {
            ans = sum;
        }
    }

    assert_eq!(ans, 21);
}
