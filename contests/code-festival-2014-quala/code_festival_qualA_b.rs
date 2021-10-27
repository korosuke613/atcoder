#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: String,
        B: u64
    }

    let A: String = a;
    let len = A.len();
    let mut index = (B as usize) % len;
    if index == 0 {
        index = len - 1;
    } else {
        index -= 1;
    }
    println!("{}", A.as_bytes()[index] as char);
}

// https://atcoder.jp/contests/code-festival-2014-quala/tasks/code_festival_qualA_b
