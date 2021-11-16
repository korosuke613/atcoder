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
        K: usize,
        A: usize
    }

    // if N == 1 {
    //     return format!("1");
    // }

    let mut answer = 0;

    let answer = N - ((K + A) % N - 1);
    // let place = not_buf_place + A - 1;
    // if place > N {
    //     answer = place % N;
    // } else {
    //     answer = place;
    // }

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
        r#"3 3 1"# => "3",
        r#"3 3 2"# => "1",
        r#"3 3 3"# => "2",
        r#"1 100 1"# => "1",
        r#"3 14 2"# => "3",
    }
}

// https://atcoder.jp/contests/abc227/tasks/abc227_a
