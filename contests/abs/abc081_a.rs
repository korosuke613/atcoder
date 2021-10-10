use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut x: u8 = 0;
    for c in s.as_str().chars() {
        if c == '1' {
            x += 1;
        }
    }

    println!("{}", x);
}
