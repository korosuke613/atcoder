#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: u32,
        m: [u32; N]
    }

    let mut x: u32 = 0;

    for subject in m {
        if subject < 80 {
            x += 80 - subject;
        }
    }

    println!("{}", x);
}

// https://atcoder.jp/contests/arc037/tasks/arc037_a
