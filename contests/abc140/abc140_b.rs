#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [u64; N],
        B: [u64; N],
        C: [u64; N-1]
    }

    let mut sutisfy = 0;
    let mut old_i = 21;
    for i in A {
        sutisfy += B[(i - 1) as usize];
        if old_i + 1 == i {
            sutisfy += C[(old_i - 1) as usize];
        }
        old_i = i;
    }

    println!("{}", sutisfy);
}

// https://atcoder.jp/contests/abc140/tasks/abc140_b
