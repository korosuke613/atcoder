#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use indexmap::IndexMap;

        // 文字列に登場する文字とその数をMapに入れる
        let mut letters = IndexMap::new();
        for ch in "a short treatise on fungi".chars() {
            *letters.entry(ch).or_insert(0) += 1;
        }

        assert_eq!(letters[&'s'], 2);
        assert_eq!(letters[&'t'], 3);
        assert_eq!(letters[&'u'], 1);
        assert_eq!(letters.get(&'y'), None);
    }
}
