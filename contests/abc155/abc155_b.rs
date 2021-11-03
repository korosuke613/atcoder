#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [u64; N]
    }

    for a in A {
        if a % 2 == 0 {
            if a % 3 == 0 || a % 5 == 0 {
                continue;
            } else {
                println!("DENIED");
                return;
            }
        }
    }

    println!("APPROVED");
    return;
}

// https://atcoder.jp/contests/abc155/tasks/abc155_b
