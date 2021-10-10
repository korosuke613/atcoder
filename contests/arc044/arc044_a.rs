#![allow(non_snake_case)]

use proconio::input;

fn get_digit(mut n: u64) -> u32 {
    let mut digit = 0;
    while n != 0 {
        n /= 10;
        digit += 1
    }
    return digit;
}

fn main() {
    input! {
        N: u64
    }

    if N == 1 {
        println!("Not Prime");
        return;
    }

    for i in 2..N {
        if N % i == 0 {
            if N == i {
                println!("Prime");
                return;
            }
        }
    }

    let digit = get_digit(N);
    let first = N - (N / (10_u64.pow(digit - 1)) * 10);
    let is_five = first == 5;
    let is_even = first % 2 == 0;
    if is_even || is_five {
        println!("Not Prime");
        return;
    }

    let mut n = N;
    let mut sum = 0;
    for i in 1..digit {
        let first = N - (N / (10_u64.pow(digit - i)) * 10);
        sum += first;
        n /= 10;
    }
    if (sum + n) % 3 == 0 {
        println!("Not Prime");
        return;
    }

    println!("Prime");
}

// https://atcoder.jp/contests/arc044/tasks/arc044_a
// 未完成
// 参考 https://atcoder.jp/contests/arc044/submissions/26498999
