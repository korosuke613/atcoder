#![allow(non_snake_case)]

use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        N: usize,
        A: [u64; N]
    }

    let mut paper: HashMap<u64, bool> = HashMap::new();

    for a in A {
        if paper.contains_key(&a) {
            paper.remove(&a);
        } else {
            paper.insert(a, true);
        }
    }

    println!("{}", paper.len())
}

// https://atcoder.jp/contests/abc073/tasks/abc073_c
