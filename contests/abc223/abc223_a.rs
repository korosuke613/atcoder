#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: u32,
    }

    if N % 100 == 0 && N > 0 {
        println!("Yes")
    } else {
        println!("No")
    }
}

// https://atcoder.jp/contests/abc223/tasks/abc223_a
