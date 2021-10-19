#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut S: String,
    }

    let mut answer: u64 = 0;
    let s = unsafe { S.as_bytes_mut() };

    loop {
        let mut success = false;
        for i in 0..(s.len() - 1) as usize {
            if s[i] as char == 'B' && s[i + 1] as char == 'W' {
                s[i] = 'W' as u8;
                s[i + 1] = 'B' as u8;
                answer += 1;
                success = true;
                break;
            }
        }
        if !success {
            break;
        }
    }

    println!("{}", answer);
}

// https://atcoder.jp/contests/agc029/tasks/agc029_a
// 未完...
