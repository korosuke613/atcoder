#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: u64,
        A: [u64; N]
    }

    let mut sum: u64 = 0;
    for a in &A {
        sum += *a as u64;
    }
    let ave: u64 = sum / N as u64;

    let mut sorted_a = A.to_vec();
    sorted_a.sort();
    let median = sorted_a[N as usize / 2 as usize];

    let X: u64 = (ave + median) / 2;

    println!("{}", X);
}

// 未完
