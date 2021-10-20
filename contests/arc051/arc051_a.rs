#![allow(non_snake_case)]

use libm::{acos, sin};
use proconio::{fastout, input};

fn in_circle(x: isize, y: isize, x1: isize, y1: isize, r: isize) -> bool {
    if x > x1 + r || x < x1 - r {
        return false;
    }

    let seata = acos(x as f64 / r as f64);
    let y_max_of_circle = (y1 + r) as f64 * sin(seata);
    let y_min_of_circle = (y1 + r) as f64 * sin(-seata);

    if y_max_of_circle >= y as f64 && y_min_of_circle <= y as f64 {
        return true;
    }

    return false;
}

#[fastout]
fn main() {
    input! {
        x1: isize,
        y1: isize,
        r: isize,
        x2: isize,
        y2: isize,
        x3: isize,
        y3: isize
    }

    if x1 - r >= x2 && x1 + r <= x3 && y1 - r >= y2 && y1 + r <= y3 {
        println!("NO");
    } else {
        println!("YES");
    }

    if in_circle(x2, y2, x1, y1, r)
        && in_circle(x3, y2, x1, y1, r)
        && in_circle(x2, y3, x1, y1, r)
        && in_circle(x3, y3, x1, y1, r)
    {
        println!("NO");
    } else {
        println!("YES");
    }
}

// https://atcoder.jp/contests/arc051/tasks/arc051_a
// 未完
