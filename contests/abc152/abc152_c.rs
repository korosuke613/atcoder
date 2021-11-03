#![allow(non_snake_case)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        N: usize,
        P: [u64; N]
    }

    let mut answer = 0;
    let mut min_P = P[0];
    for i in 0..N {
        let Pi = P[i];
        if Pi <= min_P {
            answer += 1;
            min_P = Pi;
            // println!("correct: {}", Pi);
        }
    }

    println!("{}", answer);
}

// https://atcoder.jp/contests/abc152/tasks/abc152_c

// 以下TLE
// fn main() {
//     input! {
//         N: usize,
//         P: [u64; N]
//     }
//
//     let mut answer = 0;
//     for i in 0..N {
//         let Pi = P[i];
//         let mut min_P = Pi;
//         for j in 0..i {
//             let Pj = P[j];
//             min_P = std::cmp::min(min_P, Pj);
//             if Pi != min_P {
//                 break;
//             }
//         }
//         // println!("min {}", min_P);
//         if Pi == min_P {
//             answer += 1;
//         }
//     }
//
//     println!("{}", answer);
// }
