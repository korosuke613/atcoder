#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: u32,
        M: u32,
        S: [String; N]
    }

    let mut x: u32 = 0;

    for i in 1..N {
        for j in 0..(N - i) {
            let mut diff = 0;
            for k in 0..M {
                if S[i as usize - 1].as_bytes()[k as usize]
                    != S[(i + j) as usize].as_bytes()[k as usize]
                {
                    diff += 1;
                }
            }
            if diff % 2 != 0 {
                x += 1;
            }
        }
    }

    println!("{}", x);
}

// https://atcoder.jp/contests/arc115/tasks/arc115_a
// 計算量爆発で死亡。ビット演算でゴニョゴニョすれば多分解けるのだろう
// 未完
