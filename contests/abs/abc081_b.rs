#![allow(non_snake_case)]

use proconio::source::auto::AutoSource;
use proconio::{fastout, input};
use std::cmp::min;
use std::io::Read;

#[fastout]
fn main() {
    // main関数は変更しない
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();

    println!("{}", solve(&s)); // Cmd + D または Ctrl + Dで入力終了
}

fn solve(src: &str) -> String {
    let source = AutoSource::from(src);

    input! {
        from source,
        N: usize,
        mut A: [u64; N]
    }

    let mut answer = 10_i32.pow(9);

    for a in A {
        let mut count = 0;
        let mut mut_a = a;
        loop {
            if mut_a % 2 == 0 {
                mut_a /= 2;
                count += 1;
            } else {
                break;
            }
        }
        answer = min(answer, count);
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
        r#"3
8 12 40
"# => "2",
        r#"4
5 6 8 10
"# => "0",
        r#"6
382253568 723152896 37802240 379425024 404894720 471526144
"# => "8"
    }
}

// https://atcoder.jp/contests/abs/tasks/abc081_b
