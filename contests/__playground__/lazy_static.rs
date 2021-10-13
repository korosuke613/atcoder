#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use std::collections::HashMap;

    lazy_static! {
        static ref HASHMAP: HashMap<u32, &'static str> = {
            let mut m = HashMap::new();
            m.insert(0, "foo");
            m.insert(1, "bar");
            m.insert(2, "baz");
            m
        };
        static ref COUNT: usize = HASHMAP.len();
        static ref NUMBER: u32 = times_two(21);
    }

    fn times_two(n: u32) -> u32 {
        n * 2
    }

    #[test]
    fn it_works() {
        assert_eq!(*COUNT, 3);
        assert_eq!(*HASHMAP.get(&0).unwrap(), "foo");
        assert_eq!(*NUMBER, 42)
    }
}
