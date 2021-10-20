#![allow(non_snake_case)]

use num::abs;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    }

    if abs(a - c) <= d {
        println!("Yes");
        return;
    }

    if abs(a - b) <= d && abs(b - c) <= d {
        println!("Yes");
        return;
    }

    println!("No");
}

// https://atcoder.jp/contests/abc097/tasks/abc097_a
