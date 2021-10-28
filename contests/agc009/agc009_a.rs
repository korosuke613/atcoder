#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        S: [[u64; 2]; N]
    }

    let mut reverse_S = S.clone();
    reverse_S.reverse();

    let mut add_num = 0;
    for s in reverse_S {
        let remaineder = (s[0] + add_num) % s[1];
        if remaineder == 0 {
            continue;
        }
        //print!("{} ", s[0] + add_num);
        add_num += s[1] - remaineder;
        //println!("{} {}", s[1] - remaineder, s[0] + add_num);
    }

    println!("{}", add_num);
}

// https://atcoder.jp/contests/agc009/tasks/agc009_a
