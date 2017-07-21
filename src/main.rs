extern crate ruffman;
use ruffman::{HuffmanNode, HuffmanDictionnary, BitPacker, HuffmanTree};

use std::collections::HashMap;

pub struct BitUnpacker {
    packed_bytes: Vec<u8>,
    current_byte: usize,
    current_offset: i8
}

impl BitUnpacker {
    fn new(packed_bytes: Vec<u8>) -> BitUnpacker{
        BitUnpacker {
            packed_bytes:packed_bytes,
            current_byte: 0,
            current_offset: 0,
        }
    }

    // todo: should deal with the fact that this can overflow
    fn read_bits(&mut self, n: i32) -> Vec<u8> {
        let mut reads: Vec<u8> = Vec::new();
        for i in 0..n {
            let curr_value = self.packed_bytes[self.current_byte] & (1 << self.current_offset) != 0;
            self.current_offset += 1;
            if self.current_offset > 7 {
                self.current_byte += 1;
                self.current_offset = 0;
            }
            reads.push(curr_value as u8);
        }

        reads
    }

    fn read_i32(&mut self) -> i32 {
        let bits = self.read_bits(32);
        let mut result = 0;
        for (i, v) in bits.iter().enumerate() {
            result |= v << i;
        }

        result as i32
    }

    fn read_i8(&mut self) -> i8 {
        let bits = self.read_bits(8);
        let mut result = 0;
        for (i, v) in bits.iter().enumerate() {
            result |= v << i;
        }

        result as i8
    }
}

fn compress(s:&String, t:&HuffmanDictionnary) -> Vec<u8> {
    let mut packer  = BitPacker::new();

    // First, store the table
    packer.pack_i8(t.table.keys().len() as u8); // is u8 enough ?
    for key in t.table.keys() {
        packer.pack_i8(*key as u8);
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

fn decompress(compressed: Vec<u8>) {
    let mut unpacker = BitUnpacker::new(compressed);
    let table_size = unpacker.read_i8();
    let mut map: HashMap<char, Vec<u8>> = HashMap::new();
    for i in 0..table_size {
        let curr_char = unpacker.read_i8() as u8;
        let encoding_len = unpacker.read_i8();
        let encoded_values = unpacker.read_bits(encoding_len as i32);
        map.insert(curr_char as char, encoded_values);
    }

    println!("{:?}", map);
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
    // let s = String::from_utf8_lossy(&compressed[..]);
    // print!("{}", s);

    decompress(compressed);

    // let some_bits = unpacker.read_bits(3);
    // let some_other_bits = unpacker.read_bits(8);
    // println!("{:?}", some_bits);
    // println!("{:?}", some_other_bits);


    // let mut packer: BitPacker = BitPacker::new();
    // // packer.pack_i32(132)
    // packer.pack_i8(132);
    // packer.debug();
}
