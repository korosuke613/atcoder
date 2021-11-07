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
        M: usize,
        listen_order: [u64; M]
    }

    let mut cases: Vec<u64> = vec![];
    for i in 0..N + 1 {
        cases.push(i as u64);
    }

    for l in listen_order {
        if l == cases[0] {
            continue;
        }

        let mut key = 0;
        let mut i = 0;
        for case in cases.clone() {
            if case == l {
                key = i;
                break;
            }
            i += 1;
        }

        let tmp_now_listen = cases[key];
        cases[key] = cases[0];
        cases[0] = tmp_now_listen;
    }

    let mut answer = String::new();
    let mut is_first = true;
    for case in cases {
        if is_first {
            is_first = false;
            continue;
        }
        if answer.len() == 0 {
            answer = format!("{}", case);
        } else {
            answer = format!("{}\n{}", answer, case)
        }
    }

    answer.to_string()
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
        r#"5 6
2
3
5
0
1
3"# => r#"0
5
2
4
1"#,
        r#"3 5
0
1
1
1
2"# => r#"0
1
3"#,
        r#"5 0"# => r#"1
2
3
4
5"#,
        r#"10 7
2
8
5
3
3
8
1"# => r#"8
0
5
4
3
6
7
2
9
10"#,
        r#"5 7
3
4
3
1
2
2
0"# => r#"3
1
2
4
5"#
    }
}

// https://atcoder.jp/contests/arc007/tasks/arc007_2
