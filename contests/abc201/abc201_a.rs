#![allow(non_snake_case)]

use permutohedron::LexicalPermutation;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        A: [i64; 3]
    }

    let mut B = A.clone();
    B.sort();

    loop {
        if B[2] - B[1] == B[1] - B[0] {
            println!("Yes");
            return;
        }
        if !B.next_permutation() {
            break;
        }
    }
    println!("No");
}

// https://atcoder.jp/contests/abc201/tasks/abc201_a
