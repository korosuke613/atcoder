#![allow(non_snake_case)]

use permutohedron::LexicalPermutation;
use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        N: u32,
        M: u32,
        L: u32,
        P: u32,
        Q: u32,
        R: u32,
    }

    let mut data = [P, Q, R];
    data.sort();
    let mut answer: u32 = 0;

    loop {
        let tate_num = N / data[0];
        let yoko_num = M / data[1];
        let takasa_num = L / data[2];
        let a = tate_num * yoko_num * takasa_num;
        answer = max(answer, a);

        if !data.next_permutation() {
            break;
        }
    }

    println!("{}", answer);
}

// https://atcoder.jp/contests/tenka1-2014-qualb/tasks/tenka1_2014_qualB_a
