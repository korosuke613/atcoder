#![allow(non_snake_case)]

use proconio::input;

fn main() {
    input! {
        N: u32,
        X: [[u32; 2]; N]
    }

    let mut line_length = 0;
    for x in &X {
        line_length += x[0];
    }

    let divide_second = 1000000.;

    let mut left_fired_length = 0.;
    let mut left_line_index = 0;
    let mut left_throw_length = X[0][0];
    let mut left_speed_per_nanoseconds = X[0][1] as f64 / divide_second;
    let mut right_fired_length = 0.;
    let mut right_line_index = 0;
    let mut right_throw_length = X[N as usize - 1][0];
    let mut right_speed_per_nanoseconds = X[N as usize - 1][1] as f64 / divide_second;

    loop {
        if left_fired_length + right_fired_length > line_length as f64 {
            break;
        }

        if (left_throw_length as f64) < left_fired_length {
            left_line_index += 1;
            left_throw_length += X[left_line_index][0];
            left_speed_per_nanoseconds = X[left_line_index][1] as f64 / divide_second;
        }
        left_fired_length += left_speed_per_nanoseconds;

        if (right_throw_length as f64) < right_fired_length {
            right_line_index += 1;
            right_throw_length += X[N as usize - 1 - right_line_index as usize][0];
            right_speed_per_nanoseconds =
                X[N as usize - 1 - right_line_index as usize][1] as f64 / divide_second;
        }
        right_fired_length += right_speed_per_nanoseconds;
    }

    println!("{}", left_fired_length);
}

// https://atcoder.jp/contests/abc223/tasks/abc223_c
