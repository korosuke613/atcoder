#![allow(non_snake_case)]

use libm::{fmax, tan};
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
        mut dhs: [[f64; 2]; N]
    }

    let mut answer: f64 = 0.;

    for dh in dhs {
        let d = dh[0];
        let h = dh[1];

        let katamuki: f64 = (H as f64 - h) / (D as f64 - d);
        let takasa = h - d * katamuki;
        answer = fmax(answer, takasa);
    }

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
        r#"1 1000 1000
    999 999"# => "0.0000",
    }
}

// https://atcoder.jp/contests/zone2021/tasks/zone2021_b
