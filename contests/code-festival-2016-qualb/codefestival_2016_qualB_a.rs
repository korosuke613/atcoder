#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        S: String,
    }

    let collect = "CODEFESTIVAL2016";
    let mut x = 0;

    if S == collect {
        println!("0");
        return;
    }

    for i in 0..16 {
        if S.as_bytes()[i] != collect.as_bytes()[i] {
            x += 1;
        }
    }
    println!("{}", x)
}
