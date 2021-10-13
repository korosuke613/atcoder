#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        b: char,
    }

    let mut x: char;

    match b {
        'A' => x = 'T',
        'T' => x = 'A',
        'G' => x = 'C',
        'C' => x = 'G',
        _ => x = ' ',
    }

    println!("{}", x);
}

// https://atcoder.jp/contests/abc122/tasks/abc122_a
