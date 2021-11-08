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
        mut N: usize,
    }

    let mut need_money = 0;

    let tako_set = N / 10;
    if tako_set >= 1 {
        need_money = tako_set * 100;
        N -= tako_set * 10;
    }

    if N % 10 >= 7 {
        need_money += 100;
    } else {
        need_money += N * 15;
    }

    format!("{}", need_money)
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
        r#"2"# => "30",
        r#"5"# => "75",
        r#"7"# => "100",
        r#"17"# => "200",
    }
}

// https://atcoder.jp/contests/arc008/tasks/arc008_1
