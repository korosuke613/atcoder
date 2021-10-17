#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        S: String,
    }

    let mut chars = vec![];
    chars.push(S.clone());
    for i in 1..S.len() {
        let mut tmp_S = S.clone();
        let second_S = tmp_S.split_off(i);
        chars.push(second_S + &tmp_S);
    }
    chars.sort();

    println!("{}", chars[0]);
    println!("{}", chars[S.len() - 1]);
}

// https://atcoder.jp/contests/abc223/tasks/abc223_b
