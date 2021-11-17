#![allow(non_snake_case)]

use libm::{atan, tan};
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
        D: usize,
        H: usize,
        mut dhs: [[usize; 2]; N]
    }

    dhs.sort(); // dの昇順

    let mut sieta = atan(H as f64 / D as f64);
    let mut H_now: f64 = H as f64;

    for dh in dhs {
        let d: f64 = dh[0] as f64;
        let h: f64 = dh[1] as f64;

        let height = tan(sieta) * d;
        if height.round() < h.round() {
            let local_sieta = atan((H as f64 - h) / (D as f64 - d));
            let buf = h - tan(local_sieta) * d;
            H_now += buf;
            sieta = atan(D as f64 / H_now);
        }
    }
    let answer = H_now - H as f64;

    format!("{:.4}", answer)
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
        r#"1 1000 1000
    1000 1000"# => "0.0000",
        r#"1 10 10
    3 5"# => "2.8571",
        r#"1 10 10
    3 2"# => "0.0000",
        r#"5 896 483
228 59
529 310
339 60
78 266
659 391"# => "245.3081",
    }
}

// https://atcoder.jp/contests/zone2021/tasks/zone2021_b
