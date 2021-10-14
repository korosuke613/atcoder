#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        X: char,
        Y: char
    }

    let mut x: char = '=';

    if X < Y {
        x = '<'
    } else if X > Y {
        x = '>'
    }

    println!("{}", x);
}

// https://atcoder.jp/contests/abc078/tasks/abc078_a
