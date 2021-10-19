#![allow(non_snake_case)]

use num::abs;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [[i64; 3]; N],
    }

    let mut i = 0;
    for a in A.clone() {
        let t = &a[0];
        let x = &a[1];
        let y = &a[2];

        if t < &(x + y) || t % 2 != (x + y) % 2 {
            println!("No");
            return;
        }

        if i >= 1 {
            let tt = &A[i - 1][0];
            let xx = &A[i - 1][1];
            let yy = &A[i - 1][2];
            let ans = &((t - tt) - abs((x + y) - (xx + yy)));
            if 0 > *ans || (t - tt) % 2 != abs((x + y) - (xx + yy)) % 2 {
                println!("No");
                return;
            }
        }

        i += 1;
    }

    println!("Yes");
}

// https://atcoder.jp/contests/abc086/tasks/arc089_a
