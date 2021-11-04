#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        S: usize,
        C: usize
    }

    let mut s = S;
    if S * 2 > C {
        s = C / 2;
    }

    let remaining_c = C - s * 2;
    let total = s + remaining_c / 4;

    println!("{}", total);
}

// https://atcoder.jp/contests/arc069/tasks/arc069_a
