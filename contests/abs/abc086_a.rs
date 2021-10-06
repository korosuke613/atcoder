use proconio::input;

fn main() {
    input! {
        a: u16,
        b: u16,
    }

    let x;
    if a * b % 2 == 0 {
        x = "Even";
    } else {
        x = "Odd";
    }

    println!("{}", x);
}
