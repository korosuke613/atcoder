#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [u64; N],
        B: [u64; N]
    }

    let mut left = 0;
    let mut right = 1000;

    for i in 0..N {
        left = std::cmp::max(left, A[i]);
        right = std::cmp::min(right, B[i]);
    }

    if left <= right {
        println!("{}", right - left + 1);
    } else {
        println!("0");
    }
}

// https://atcoder.jp/contests/abc199/tasks/abc199_b
