extern crate ruffman;
use ruffman::{HuffmanNode, HuffmanDictionnary, BitPacker, HuffmanTree};

fn compress(s:&String, t:&HuffmanDictionnary) -> Vec<u8> {
    let mut packer  = BitPacker::new();

    // First, store the table
    packer.pack_i8(t.table.keys().len() as u8); // is u8 enough ?
    for key in t.table.keys() {
        packer.pack_i8(t.table[&key].len() as u8);
        packer.pack_bits(&t.table[&key]);
    }

    // Then, store the message
    packer.pack_i32(s.len() as u32);
    for c in s.chars() {
        let ref v = t.table[&c];
        packer.pack_bits(v);
        // println!("{}, {:?}", c, v);
    }
    packer.debug();

    packer.flush()
}

fn main() {
    let original = String::from("abbcccc");
    // let mut tree = HuffmanTree::new();
    // let root = tree.build(&original);
    // println!("{:#?}", root);

    let a = HuffmanNode::new_leaf('a', 1);
    let b = HuffmanNode::new_leaf('b', 2);
    let c = HuffmanNode::new_leaf('c', 4);
    let ab = HuffmanNode::new_node(&a, &b);
    let tree = HuffmanNode::new_node(&ab, &c);

    let mut table = HuffmanDictionnary::new();
    table.build_table(&tree);

    let compressed = compress(&original, &table);
    let s = String::from_utf8_lossy(&compressed[..]);
    print!("{}", s);

    // let mut packer: BitPacker = BitPacker::new();
    // // packer.pack_i32(132)
    // packer.pack_i8(132);
    // packer.debug();
}
