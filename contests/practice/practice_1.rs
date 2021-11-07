use proconio::input;
use proconio::source::auto::AutoSource;

macro_rules! stdin {
    () => {{
        use std::io::Read;
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    }};
}

fn main() {
    println!("{}", solve(&stdin!()));
}

fn solve(src: &str) -> String {
    let source = AutoSource::from(src);

    input! {
        from source,
        a: u16,
        b: u16,
        c: u16,
        s: String
    }

    let x = a + b + c;

    format!("{} {}", x, s)
}

#[cfg(test)]
mod test {
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

    use crate::solve;

    test! {
        r#"1
2 3
test"# => "6 test",

        r#"72
128 256
myonmyon
"# => "456 myonmyon"
    }
}
