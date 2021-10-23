#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        S: String,
    }

    if S.ends_with("er") {
        println!("er");
    } else if S.ends_with("ist") {
        println!("ist");
    }
    return;
}

// https://atcoder.jp/contests/abc224/tasks/abc224_a
