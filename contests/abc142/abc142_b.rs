use proconio::input;

fn main() {
    input! {
        N: u32,
        K: u32,
        h: [u32; N]
    }

    let mut x: u32 = 0;

    for a in h {
        if a >= K {
            x += 1;
        }
    }

    println!("{}", x);
}
