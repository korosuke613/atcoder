#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: u32,
        M: u32
    }

    if M == N {
        println!("Yes");
    } else {
        println!("No");
    }
}
