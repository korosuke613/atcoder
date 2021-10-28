#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        S: [String; N]
    }
    let month_max_day = [31, 29, 31, 30, 31, 30, 31, 31, 29, 31, 30, 31];
    let month_first_youbi = [0, 3, 4, 0, 2, 5, 0, 3, 6, 1, 4, 6];

    let mut max_length_rest = 2;
    for s_ in S {
        let s: String = s_.clone();
        let md: Vec<&str> = s.split("/").collect();
        let m = md[0].parse::<u64>().unwrap();
        let d = md[1].parse::<u64>().unwrap();
        println!("{} {}", m, d);

        let mut youbi = (month_first_youbi[m as usize - 1] + d - 1) % 7;
        if youbi == 5 || youbi == 1 {
            max_length_rest = 3;
        }
    }
}

// https://atcoder.jp/contests/arc010/tasks/arc010_2
// 未完
// 既存の祝日や祝日が連続する場合や、月をまたぐ場合を考えないといけない
