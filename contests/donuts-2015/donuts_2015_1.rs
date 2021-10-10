#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        R: u32,
        D: u32
    }

    let a: f64 = (R * R) as f64 * std::f64::consts::PI;
    let l: f64 = (D * 2) as f64 * std::f64::consts::PI;
    let x = a * l;
    println!("{}", x);
}

// https://atcoder.jp/contests/donuts-2015/tasks/donuts_2015_1
