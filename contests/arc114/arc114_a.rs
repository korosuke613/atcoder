#![allow(non_snake_case)]

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        N: u32,
        X: [u32; N]
    }

    let mut divide_nums = Vec::new();

    for x in X {
        let mut b = 2;
        loop {
            if x % b == 0 {
                divide_nums.push(b);
                break;
            }
            b += 1;
        }
    }

    let uniq_divide_nums: HashSet<u32> = divide_nums.into_iter().collect();
    let mut a: u32 = 1;
    for uniq_divide_num in uniq_divide_nums {
        a *= uniq_divide_num
    }

    println!("{}", a);
}

// 未完成
// 参考 https://atcoder.jp/contests/arc114/submissions/26407592
