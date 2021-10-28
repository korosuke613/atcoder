#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        R: usize,
    }

    if R < 1200 {
        println!("ABC");
    } else if R < 2800 {
        println!("ARC");
    } else {
        println!("AGC");
    }
}

// https://atcoder.jp/contests/abc104/tasks/abc104_a
