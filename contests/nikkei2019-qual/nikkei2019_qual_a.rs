#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: i32,
        A: i32,
        B: i32
    }

    let mut x: i32 = 0;
    let mut y: i32 = 0;

    if A < B {
        x = A;
        if A - (N - B) < 0 {
            y = 0
        } else {
            y = A - (N - B);
        };
    } else {
        x = B;
        if B - (N - A) < 0 {
            y = 0
        } else {
            y = B - (N - A);
        };
    }

    println!("{} {}", x, y);
}

// https://atcoder.jp/contests/nikkei2019-qual/tasks/nikkei2019_qual_a
