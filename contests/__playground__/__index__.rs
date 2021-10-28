use crate::text_io::text_io_read;

mod hello_world;
mod indexmap;
mod lazy_static;
mod petgraph;
mod superslice;
mod text_io;

fn main() {
    text_io_read();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
