#![allow(non_snake_case)]
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        N: usize,
        Q: usize,
    };

    let f = |a: &usize, b: &usize| *a.max(b);

    let mut seg = SegmentTree::new(N, 0, f);
    for _ in 0..Q {
        input! {
            q: usize
        };

        match q {
            1 => {
                input! {
                    k: Usize1,
                    v: usize,
                }
                seg.update(k, v);
            }
            2 => {
                input! {
                    l: Usize1,
                    r: Usize1,
                }
                println!("{}", seg.query(l, r))
            }
            _ => unreachable!(),
        }
    }
}

pub struct SegmentTree<T, F> {
    n: usize,
    seg: Vec<T>,
    op: F,
    init_value: T,
}

impl<T, F> SegmentTree<T, F>
where
    T: Copy,
    F: Fn(&T, &T) -> T,
{
    pub fn new(n: usize, init_value: T, op: F) -> Self {
        assert!(n > 0);
        let m = n.next_power_of_two();
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
            self.init_value
        } else if left <= left_bound && right_bound <= right {
            self.seg[k]
        } else {
            let mid = (left_bound + right_bound) >> 1;
            let x = self.query_range(left, right, 2 * k + 1, left_bound, mid);
            let y = self.query_range(left, right, 2 * k + 2, mid, right_bound);
            (self.op)(&x, &y)
        }
    }
}
