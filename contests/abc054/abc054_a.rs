#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        A: usize,
        B: usize,
    }

    if A == B {
        println!("Draw");
    } else if A == 1 {
        println!("Alice");
    } else if B == 1 || B > A {
        println!("Bob");
    } else if A > B {
        println!("Alice");
    } else {
        println!("Bob");
    }
}

// https://atcoder.jp/contests/abc054/tasks/abc054_a
