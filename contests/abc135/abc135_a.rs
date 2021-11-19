#![allow(non_snake_case)]

use proconio::source::auto::AutoSource;
use proconio::{fastout, input};
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
        A: usize,
        B: usize,
    }

    if (A + B) % 2 == 0 {
        format!("{}", (A + B) / 2)
    } else {
        format!("IMPOSSIBLE")
    }
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
        r#"2 16"# => "9",
        r#"0 3"# => "IMPOSSIBLE",
        r#"998244353 99824435"# => "549034394",
    }
}

// https://atcoder.jp/contests/abc135/tasks/abc135_a
