#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize,
    }

    let answer = N % K;

    println!("{}", answer);
}

// https://atcoder.jp/contests/diverta2019-2/tasks/diverta2019_2_a
