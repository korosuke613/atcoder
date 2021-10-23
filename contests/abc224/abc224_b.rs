#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        A: [[u64; W]; H]
    }

    let mut left = 0;
    let mut right = 0;
    for i in 0..H {
        for j in 0..W {
            if (i == 0 && j == 0) || (i == H - 1 && j == W - 1) {
                // 先頭と最後
                left += A[i][j] * (W - 1 - j) as u64 * (H - 1 - i) as u64
                    + A[i][j] * i as u64 * j as u64;
            } else if (i == 0 && j == W - 1) || (i == H - 1 && j == 0) {
                // 先頭と最後
                right += A[i][j] * (W - 1 - j) as u64 + A[i][j] * (H - 1 - i) as u64;
            } else if i == 0 || i == H - 1 {
                // 最初行と最終行
                left += A[i][j] * (W - 1 - j) as u64 * (H - 1 - i) as u64
                    + A[i][j] * i as u64 * j as u64;
                right += A[i][j] * (W - 1 - j) as u64;
            } else if j == 0 || j == W - 1 {
                // 最初行と最終行
                left += A[i][j] * (W - 1 - j) as u64 * (H - 1 - i) as u64
                    + A[i][j] * i as u64 * j as u64;
                right += A[i][j] * (H - 1 - i) as u64;
            } else {
                // 基本
                left += A[i][j] * (W - 1 - j) as u64 * (H - 1 - i) as u64
                    + A[i][j] * i as u64 * j as u64;
                right += A[i][j] * (W - 1 - j) as u64 + A[i][j] * (H - 1 - i) as u64;
            }
        }
    }

    if left <= right {
        println!("Yes");
    } else {
        println!("No");
    }
}

// https://atcoder.jp/contests/abc224/tasks/abc224_b
