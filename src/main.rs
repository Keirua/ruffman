extern crate ruffman;

use std::collections::HashMap;
use ruffman::{HuffmanNode, HuffmanDictionnary, BitPacker, HuffmanTree};

fn compress(s:&String, t:&HuffmanDictionnary) {
    let mut packer  = BitPacker::new();
    for c in s.chars() {
        let ref v = t.table[&c];
        packer.pack_bits(v);
        println!("{}, {:?}", c, v);
    }
    packer.debug();
}

fn main() {
    let original = String::from("abbcccc");
    let mut tree = HuffmanTree::new();
    
    let root = tree.build(&original);
    println!("{:#?}", root);

    let a = HuffmanNode::new_leaf('a', 1);
    let b = HuffmanNode::new_leaf('b', 2);
    let c = HuffmanNode::new_leaf('c', 4);
    let ab = HuffmanNode::new_node(&a, &b);
    let tree = HuffmanNode::new_node(&ab, &c);

    let mut table = HuffmanDictionnary::new();
    table.build_table(&tree);

    compress(&original, &table);
}
