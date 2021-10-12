#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: u32,
        M: u32,
        L: u32,
        P: u32,
        Q: u32,
        R: u32,
    }

    let tate_num_a = N / P;
    let yoko_num_a = M / Q;
    let takasa_num_a = L / R;
    let a = tate_num_a * yoko_num_a * takasa_num_a;

    let tate_num_b = N / Q;
    let yoko_num_b = M / P;
    let takasa_num_b = L / R;
    let b = tate_num_b * yoko_num_b * takasa_num_b;

    let tate_num_c = N / P;
    let yoko_num_c = M / R;
    let takasa_num_c = L / Q;
    let c = tate_num_c * yoko_num_c * takasa_num_c;

    let tate_num_d = N / R;
    let yoko_num_d = M / Q;
    let takasa_num_d = L / P;
    let d = tate_num_d * yoko_num_d * takasa_num_d;

    let tate_num_e = N / R;
    let yoko_num_e = M / P;
    let takasa_num_e = L / Q;
    let e = tate_num_e * yoko_num_e * takasa_num_e;

    let tate_num_f = N / Q;
    let yoko_num_f = M / R;
    let takasa_num_f = L / P;
    let f = tate_num_f * yoko_num_f * takasa_num_f;

    match std::cmp::max(
        std::cmp::max(std::cmp::max(std::cmp::max(std::cmp::max(a, b), c), d), e),
        f,
    ) {
        a => println!("{}", a),
        b => println!("{}", b),
        c => println!("{}", c),
        d => println!("{}", d),
        e => println!("{}", e),
        f => println!("{}", f),
        _ => println!("0"),
    }
}

// https://atcoder.jp/contests/tenka1-2014-qualb/tasks/tenka1_2014_qualB_a
