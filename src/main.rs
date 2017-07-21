extern crate ruffman;

fn main() {
    let original = "abbcccc";
    let mut packer = ruffman::BitPacker::new();
    // packer.pack_bit(1);
    // packer.pack_bit(0);
    // packer.pack_bit(1);
    // packer.pack_bit(1);

    let bits = vec![1,0,0,0,0,0,0,1,1];
    packer.pack_bits(&bits);
    println!("original: {}", original);
    println!("{:?}", packer);
}
