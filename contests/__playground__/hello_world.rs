#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut a = vec![];
        a.push("Hello, world!");

        println!("{:?}", a);
        a[0].as_bytes()
            .iter()
            .for_each(|x| print!("{}", *x as char));
    }
}
