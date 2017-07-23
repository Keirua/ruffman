extern crate ruffman;
use ruffman::{compress, decompress};

fn main() {
    let original = String::from("abbcccc");

    let compressed = compress(&original);
    print!("{}", decompress(compressed));

    // let s = String::from_utf8_lossy(&decompressed[..]);
    // print!("{}", s);

    // let mut packer: BitPacker = BitPacker::new();
    // // packer.pack_i32(132)
    // packer.pack_i8(132);
    // packer.debug();
}
