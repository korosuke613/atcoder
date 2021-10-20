#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize
    }

    let mut chars = vec![];

    let mut tmp_s: String = "".to_string();
    for _i in 0..a {
        tmp_s = format!("{}{}", tmp_s, b.to_string());
    }
    chars.push(tmp_s);

    let mut tmp_s: String = "".to_string();
    for _i in 0..b {
        tmp_s = format!("{}{}", tmp_s, a.to_string());
    }
    chars.push(tmp_s);

    chars.sort();

    println!("{}", chars[0]);
}

// https://atcoder.jp/contests/abc152/tasks/abc152_b
