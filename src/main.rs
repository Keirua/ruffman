extern crate ruffman;
use ruffman::{compress, decompress};

fn main() {
    let original = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit.");

    let compressed = compress(&original);
    print!("{}", decompress(compressed));
}
