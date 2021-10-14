#![allow(non_snake_case)]

use libm::pow;
use proconio::input;

fn main() {
    input! {
        X: i64,
    }

    let mut a: i64 = 1;
    let mut b: i64 = -1;
    loop {
        loop {
            if a.pow(5) - b.pow(5) == X {
                println!("{} {}", a, b);
                return;
            }
            b -= 1;
        }
        a += 1;
    }
}

// https://atcoder.jp/contests/abc166/tasks/abc166_d
// 未完
// 参考：https://atcoder.jp/contests/abc166/submissions/26560916
