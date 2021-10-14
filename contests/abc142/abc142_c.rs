#![allow(non_snake_case)]

use indexmap::IndexMap;
use proconio::input;

fn main() {
    input! {
        N: u32,
        A: [u32; N]
    }

    let mut seito = IndexMap::new();
    let mut index = 1;
    for a in A {
        *seito.entry(a).or_insert(0) = index;
        index += 1;
    }

    for i in 1..(N + 1) {
        print!("{} ", seito[&i]);
    }
    println!();
}

// https://atcoder.jp/contests/abc142/tasks/abc142_c
