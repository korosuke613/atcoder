#![allow(non_snake_case)]

use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        AB: [[usize; 2]; M]
    }

    let mut max_need_road = N - 1;

    let mut already = HashMap::new();
    for ab in AB {
        if already.contains_key(&ab[0]) && already.contains_key(&ab[1]) {
            continue;
        }
        already.insert(ab[0], true);
        already.insert(ab[1], true);
        max_need_road -= 1;
    }

    println!("{}", max_need_road);
}

// https://atcoder.jp/contests/arc032/tasks/arc032_2
// なんかテストケース2問だけ間違えるけどよくわからん。
