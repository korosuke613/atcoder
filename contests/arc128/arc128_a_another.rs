#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: u64,
        A: [u64; N as u64]
    }

    let mut answer = vec![];
    for _ in 0..N {
        answer.push(0);
    }

    for i in 0..(N - 1) as u64 {
        if A[i as usize] > A[i as usize + 1] {
            answer[i as usize] = 1;
            answer[i as usize + 1] = 1;
        }
    }

    for a in answer {
        print!("{} ", a);
    }
    println!();
}

// https://atcoder.jp/contests/arc128/tasks/arc128_a
// 参考：https://atcoder.jp/contests/arc128/submissions/26599196
