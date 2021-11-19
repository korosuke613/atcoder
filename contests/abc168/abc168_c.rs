#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::f64::consts::PI;
use std::io::Read;

use libm::{cos, sqrt};
use proconio::source::auto::AutoSource;
use proconio::{fastout, input};

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
        H_LENGTH: usize,
        M_LENGTH: usize,
        H: usize,
        M: usize,
    }

    let H_LENGTH: usize = H_LENGTH;
    let M_LENGTH: usize = M_LENGTH;

    let h_angle = 360. / 12. * H as f64 + 360. / 12. / 60. * M as f64;
    let m_angle = 360. / 60. * M as f64;
    let angle = (h_angle - m_angle).abs();

    let a2plusb2 = (H_LENGTH.pow(2) + M_LENGTH.pow(2)) as f64;
    let ab2 = (2 * H_LENGTH * M_LENGTH) as f64;
    let cosC = cos(angle * (PI / 180.));

    let answer = sqrt(a2plusb2 - ab2 * cosC);

    format!("{:.10}", answer)
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
        r#"3 4 9 0"# => "5.0000000000",
        r#"3 4 10 40"# => "4.5642571943",
        r#"3 3 0 0"# => "0.0000000000",
        // r#"3 4 0 40"# => "0.0000000000",
    }
}

// https://atcoder.jp/contests/abc168/tasks/abc168_c
