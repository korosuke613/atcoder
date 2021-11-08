#![allow(non_snake_case)]

use petgraph::graph::UnGraph;
use proconio::source::auto::AutoSource;
use proconio::{fastout, input};
use std::collections::{BTreeMap, HashMap};
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
        B: [usize; N-1]
    }

    let mut answer = 0;
    let mut employees = HashMap::new();
    let g = UnGraph::<i32, ()>::from_edges(&[(1, 2), (2, 3), (3, 4), (1, 4)]);

    for b in B {
        employees.entry(b).and_modify(|e| *e += 1).or_insert(1);
    }

    format!("{:?}", employees)
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
        r#"1 1"# => "2",
    }
}

// https://atcoder.jp/contests/abc026/tasks/abc026_c
