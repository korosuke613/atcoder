#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        S: String,
    }

    let mut x: u32 = 0;

    let rev_s: String = S.chars().rev().collect::<String>();
    let s_len = S.len();

    for i in 0..(s_len / 2) {
        if S.as_bytes()[i] != rev_s.as_bytes()[i] {
            x += 1;
        }
    }

    println!("{}", x);
}

// https://atcoder.jp/contests/abc147/tasks/abc147_b
