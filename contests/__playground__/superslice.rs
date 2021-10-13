#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use superslice::*;

        let b = [1, 3];
        assert_eq!(b.lower_bound(&1), 0);
        assert_eq!(b.upper_bound(&1), 1);
        assert_eq!(b.equal_range(&3), 1..2);
    }
}
