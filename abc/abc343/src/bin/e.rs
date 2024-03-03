#![allow(non_snake_case)]
use std::isize;

use proconio::input;

fn main() {
    input! {
        V1: usize,
        V2: usize,
        V3: usize
    };

    if V1 + V2 * 2 + V3 * 3 != 7 * 7 * 7 * 3 {
        println!("No");
        return;
    }

    let a1 = 0;
    let b1 = 0;
    let c1 = 0;

    for a2 in 0..=7 {
        for b2 in a2..=a2 + 7 {
            for c2 in b2..=b2 + 7 {
                for a3 in -7..=7 {
                    for b3 in -7..=7 {
                        for c3 in -7..=7 {
                            let x3 = (a3 + 7).min((a2 + 7).min(a1 + 7)) - (a3.max(a2.max(a1)));
                            let y3 = (b3 + 7).min((b2 + 7).min(b1 + 7)) - (b3.max(b2.max(b1)));
                            let z3 = (c3 + 7).min((c2 + 7).min(c1 + 7)) - (c3.max(c2.max(c1)));
                            let v3 = x3 * y3 * z3;

                            if v3 != V3 as isize {
                                continue;
                            }
                        }
                    }
                }
            }
        }
    }
}
