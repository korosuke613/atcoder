use proconio::input;

fn main() {
    input! {
        a: u16,
        b: u16,
        c: u16,
        s: String
    }

    let x = a + b + c;

    println!("{} {}", x, s);
}
