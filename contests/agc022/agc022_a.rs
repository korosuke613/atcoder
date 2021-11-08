#![allow(non_snake_case)]

use proconio::source::auto::AutoSource;
use proconio::{fastout, input};
use std::collections::HashSet;
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
        S: String
    }
    let S: String = S;

    let mut words: HashSet<u8> = HashSet::new();
    let start_char = "a".as_bytes()[0];
    let end_char = "z".as_bytes()[0];

    for c in start_char..end_char {
        words.insert(c);
    }

    if S == "zyxwvutsrqponmlkjihgfedcba" {
        return format!("-1");
    }

    for s in S.chars() {
        words.remove(&s.to_string().as_bytes()[0]);
    }

    let sorted_words = words.into_iter().collect::<Vec<_>>();
    format!("{}{}", S, sorted_words[0].to_string())
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
        r#"atcoder"# => "atcoderb",
        r#"abc"# => "abcd",
        r#"zyxwvutsrqponmlkjihgfedcba"# => "-1",
        r#"abcdefghijklmnopqrstuvwzyx"# => "abcdefghijklmnopqrstuvx",
    }
}

// https://atcoder.jp/contests/agc022/tasks/agc022_a
