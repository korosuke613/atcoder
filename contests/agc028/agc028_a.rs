#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        S: String,
        T: String
    }
    let S: String = S;
    let T: String = T;
    let mut is_X = false;
    let lcm = num::integer::lcm(N, M);
    let mut i = lcm;

    // println!("{}", num::integer::lcm(N, M));
    // println!("{}", num::integer::gcd(N, M));
    // println!("{:?}", num::integer::gcd_lcm(N, M));

    if S.as_bytes()[0] != T.as_bytes()[0] {
        println!("{}", -1);
        return;
    }

    loop {
        if i > 10_i32.pow(5) as usize {
            break;
        }
        let mut good_str: [char; 10_i32.pow(5) as usize] = ['\0'; 10_i32.pow(5) as usize];

        for j in 0..i {
            if j == 0 {
                good_str[0] = S.as_bytes()[0] as char
            }

            let mut is_continue = false;
            if j <= N - 1 {
                good_str[i / N * j] = S.as_bytes()[j] as char;
                is_continue = true;
            }
            if j <= M - 1 {
                good_str[i / M * j] = T.as_bytes()[j] as char;
                is_continue = true;
            }

            if !is_continue {
                break;
            }
        }

        i += lcm;
    }
}

// https://atcoder.jp/contests/agc028/tasks/agc028_a
// 未完
// https://atcoder.jp/contests/agc028/submissions/26821916
