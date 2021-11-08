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
        N: usize,
    }

    let mut answer = "Yay!";
    let add_tax = N as f64 * 1.08;

    if add_tax.round() == 206. {
        answer = "so-so"
    } else if add_tax > 206. {
        answer = ":("
    }

    format!("{}", answer)
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
        r#"180"# => "Yay!",
        r#"200"# => ":(",
        r#"191"# => "so-so",
    }
}

// https://atcoder.jp/contests/abc206/tasks/abc206_a
