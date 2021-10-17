#[allow(unused_imports)]
use proconio::{input, marker::*};
fn main() {
    input! {
        n:usize,
        abs:[(f64,f64);n],
    }
    let mut tot = 0.;
    for &(a, b) in &abs {
        tot += a / b;
    }
    let mut ans = 0.;
    let mut cur = 0.;
    for &(a, b) in &abs {
        if cur + a / b > tot / 2. {
            ans += (tot / 2. - cur) * b;
            break;
        } else {
            ans += a;
            cur += a / b;
        }
    }
    println!("{}", ans);
}

// 参考: https://atcoder.jp/contests/abc223/submissions/26634845
