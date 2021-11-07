#![allow(non_snake_case)]

use proconio::source::auto::AutoSource;
use proconio::{fastout, input};
use std::collections::HashMap;
use std::io::Read;

#[fastout]
fn main() {
    // main関数は変更しない
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();

    println!("{}", solve(&s));
}

fn solve(src: &str) -> String {
    let source = AutoSource::from(src);

    input! {
        from source,
        s: String
    }

    let mut answer: HashMap<String, bool> = HashMap::new();

    let S: String = s;
    let mut tmp_s = String::new();
    for s in S.chars() {
        let key = format!("{}{}", tmp_s, s.to_string().as_str());
        if answer.contains_key(&*key.to_string()) {
            tmp_s = key;
            continue;
        }
        answer.insert(key.to_string(), true);
        tmp_s = String::new();
    }

    format!("{}", answer.len())
}

// ここから上を提出してください
// 以下テストコード

#[cfg(test)]
mod test {
    use crate::solve;

    macro_rules! test {
        ($($input:expr => $output:expr),* $(,)*) => {
            #[test]
            fn solve_test() {
                $(
                    assert_eq!(solve($input), $output);
                )*
            }
        };
    }

    test! {
        r#"aabbaa"# => "4",
        r#"aaaccacabaababc"# => "12",
    }
}

// https://atcoder.jp/contests/agc037/tasks/agc037_a
