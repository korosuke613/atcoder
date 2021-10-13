pub fn text_io_read() {
    use text_io::read;

    let i: i32 = read!();
    println!("{}", i);
}
