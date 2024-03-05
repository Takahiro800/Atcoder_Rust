#![allow(non_snake_case)]
use std::fmt::Debug;

use proconio::input;

fn main() {
    input! {
        N: usize,
        Q: usize,
    }

    let f = |a: &usize, b: &usize| (a + b);

    let mut seg = SegmentTree::new(N, 0, f);

    for _ in 0..Q {
        input! { q: usize};
        match q {
            1 => {
                input! {
                    p: usize,
                    x: usize,
                }
                seg.update(p, x);
            }
            2 => {
                input! {
                    l: usize,
                    r: usize
                }
                println!("{}", seg.query(l, r))
            }
            _ => unreachable!(),
        }
    }
}

// -------------------- begin segment tree -----------------------------
pub struct SegmentTree<T, F> {
    n: usize,
    seg: Vec<T>,
    op: F,
    init_value: T,
}

impl<T, F> SegmentTree<T, F>
where
    T: Copy + Debug,
    F: Fn(&T, &T) -> T,
{
    pub fn new(n: usize, init_value: T, op: F) -> Self {
        assert!(n > 0);
        // let m = n.next_power_of_two();
        let mut m = 1;
        while m <= n {
            m <<= 1;
        }
        SegmentTree {
            seg: vec![init_value; 2 * m],
            n: m,
            op,
            init_value,
        }
    }

    pub fn update(&mut self, k: usize, value: T) {
        let mut k = k + self.n - 1;
        self.seg[k] = value;

        while k > 0 {
            k = (k - 1) >> 1;
            self.seg[k] = (self.op)(&self.seg[k * 2 + 1], &self.seg[k * 2 + 2]);
        }
    }

    pub fn query(&self, left: usize, right: usize) -> T {
        assert!(left < right);
        self.query_range(left, right, 0, 0, self.n)
    }

    fn query_range(
        &self,
        left: usize,
        right: usize,
        k: usize,
        left_bound: usize,
        right_bound: usize,
    ) -> T {
        if right_bound <= left || right <= left_bound {
            return self.init_value;
        }
        if left <= left_bound && right_bound <= right {
            return self.seg[k];
        }

        let mid = (left_bound + right_bound) >> 1;
        let x = self.query_range(left, right, (k << 1) + 1, left_bound, mid);
        let y = self.query_range(left, right, (k << 1) + 2, mid, right_bound);
        (self.op)(&x, &y)
    }
}
// -------------------- end segment tree -----------------------------
