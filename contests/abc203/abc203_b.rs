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
        K: usize
    }

    let mut answer = 0;

    for n in 0..N {
        for k in 0..K {
            let room_number = format!("{}0{}", n + 1, k + 1);
            answer += room_number.parse::<u64>().unwrap();
        }
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
        r#"1 2"# => "203",
        r#"3 3"# => "1818",
    }
}

// https://atcoder.jp/contests/abc203/tasks/abc203_b
