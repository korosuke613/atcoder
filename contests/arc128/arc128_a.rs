#![allow(non_snake_case)]

use indexmap::IndexMap;
use proconio::input;
use std::collections::HashMap;
use std::iter::Map;

fn main() {
    input! {
        N: u64,
        A: [u64; N]
    }

    let mut gold: u64 = 1;
    let mut silver: u64 = 0;
    let mut rank = vec![];
    let mut max_local_rank_gold = vec![];
    let mut max_local_rank_silver = vec![];
    for i in -1..(N as i64) {
        let mut local_ranks = vec![];
        let mut max_gold = HashMap::new();
        max_gold.insert("number", 0.);
        max_gold.insert("index", 0.);
        let mut max_silver = HashMap::new();
        max_silver.insert("number", 0.);
        max_silver.insert("index", 0.);
        for j in (i + 1) as u64..N {
            let mut local_rank = [0., 0.];
            // 金の評価
            if i == -1 {
                // 初日は銀に交換することしかできないため、金の評価は0とする
                local_rank[0] = 0.;
            } else {
                local_rank[0] = A[i as usize] as f64 / A[j as usize] as f64;
            }

            // 銀の評価
            if j == N - 1 {
                // 最終日は金に交換することしかできないため、銀の評価は0とする
                local_rank[1] = 0.;
            } else {
                if i == -1 {
                    local_rank[1] = 1. * A[j as usize] as f64;
                } else {
                    local_rank[1] = A[i as usize] as f64 * A[j as usize] as f64;
                }
            }

            if max_gold["number"] < local_rank[0] {
                max_gold.insert("number", local_rank[0]);
                max_gold.insert("index", j as f64);
            }
            if max_silver["number"] < local_rank[1] {
                max_silver.insert("number", local_rank[1]);
                max_silver.insert("index", j as f64);
            }
            local_ranks.push(local_rank);
        }
        max_local_rank_gold.push(max_gold);
        max_local_rank_silver.push(max_silver);
        rank.push(local_ranks);
    }

    let mut x = vec![];
    let mut next_index: u64 = max_local_rank_silver[0]["index"] as u64;
    let mut old_index: u64 = 0;
    for i in 0..N {
        if i < next_index {
            x.push(0);
            continue;
        }
        if gold != 0 {
            // 金を持っている場合
            if max_local_rank_silver[old_index as usize]["index"] as u64 == next_index {
                silver = gold * A[i as usize];
                gold = 0;
                next_index = max_local_rank_gold[i as usize + 1]["index"] as u64;
                old_index = i + 1;
                x.push(1);
                continue;
            }
        } else {
            if max_local_rank_gold[old_index as usize]["index"] as u64 == next_index {
                gold = silver / A[i as usize];
                silver = 0;
                next_index = max_local_rank_silver[i as usize + 1]["index"] as u64;
                old_index = i + 1;
                x.push(1);
                continue;
            }
        }
        x.push(0);
    }

    for answer in x {
        print!("{} ", answer)
    }
    println!();
}

// https://atcoder.jp/contests/arc128/tasks/arc128_a
// 未完
// 参考: https://atcoder.jp/contests/arc128/submissions/26599196
