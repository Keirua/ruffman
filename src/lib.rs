extern crate bitpacker;
use std::collections::HashMap;
use bitpacker::{BitPacker, BitUnpacker};

#[derive(Debug)]
pub struct HuffmanNode {
    pub key: Option<char>,
    pub value: usize,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    fn new_leaf(key: char, value: usize) -> HuffmanNode {
        HuffmanNode {
            key: Some(key),
            value: value,
            left: None,
            right: None,
        }
    }

    fn new_node(left: Box<HuffmanNode>, right: Box<HuffmanNode>) -> HuffmanNode {
        HuffmanNode {
            key: None,
            value: left.value + right.value,
            left: Some(left),
            right: Some(right),
        }
    }

    fn is_leaf(&self) -> bool {
        self.key.is_some()
    }
}

fn find_smallest_node(nodes: &Vec<Box<HuffmanNode>>) -> usize {
    let mut pos: usize = 0;
    for i in 0..nodes.len() {
        if nodes[pos].value > nodes[i].value {
            pos = i;
        }
    }
    pos
}


fn count_chars(original: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for key in original.chars() {
        *map.entry(key).or_insert(0) += 1;
    }

    map
}

pub fn build_tree(original: &str) -> Box<HuffmanNode> {
    let hash = count_chars(original);
    let mut nodes : Vec<Box<HuffmanNode>> = hash.into_iter().map(|t| Box::new(HuffmanNode::new_leaf(t.0, t.1))).collect();

    while nodes.len() > 1 {
        let idx_left = find_smallest_node(&nodes);
        let left = nodes.remove(idx_left);
        let idx_right = find_smallest_node(&nodes);
        let right = nodes.remove(idx_right);

        nodes.push(Box::new(HuffmanNode::new_node(left, right)));
    }

    nodes.remove(0)
}

struct HuffmanDictionnary {
    table: HashMap<char, Vec<u8>>
}

impl HuffmanDictionnary {
    fn new () -> HuffmanDictionnary {
        HuffmanDictionnary {
            table: HashMap::new()
        }
    }

    fn build_table (&mut self, root: &Option<Box<HuffmanNode>>)  {
        let v:Vec<u8> = Vec::new();
        self.navigate(&root, v);

        println!("Built table:\n{:#?}", self.table);
    }

    fn navigate (&mut self, node_option: &Option<Box<HuffmanNode>>, v:Vec<u8>) {
        match *node_option {
            Some(ref node) => {
                if node.is_leaf() {
                    self.table.entry(node.key.unwrap()).or_insert(v);
                }
                else {
                    let mut vl = v.clone();
                    vl.push(0);
                    let mut vr = v.clone();
                    vr.push(1);
                    self.navigate(&node.left, vl);
                    self.navigate(&node.right, vr);
                }
            }
            None => {}
        }
    }
}

pub fn compress(original:&String) -> Vec<u8> {
    let root = build_tree(&original);

    let mut table = HuffmanDictionnary::new();
    table.build_table(&Some(root));

    let mut packer  = BitPacker::new();

    // First, store the table
    packer.pack_i8(table.table.keys().len() as u8); // is u8 enough ?
    for key in table.table.keys() {
        packer.pack_i8(*key as u8);
        packer.pack_i8(table.table[&key].len() as u8);
        packer.pack_bits(&table.table[&key]);
    }

    // Then, store the message
    packer.pack_i32(original.len() as u32);
    for c in original.chars() {
        let ref v = table.table[&c];
        packer.pack_bits(v);
    }
    packer.debug();
    packer.flush()
}

pub fn decompress(compressed: Vec<u8>) -> String {
    let mut unpacker = BitUnpacker::new(compressed);
    let table_size = unpacker.read_i8();
    let mut map: HashMap<char, Vec<u8>> = HashMap::new();
    for _ in 0..table_size {
        let curr_char = unpacker.read_i8() as u8;
        let encoding_len = unpacker.read_i8();
        let encoded_values = unpacker.read_bits(encoding_len as i32);
        map.insert(curr_char as char, encoded_values);
    }

    let message_length = unpacker.read_i32();

    let mut message:String = String::from("");
    println!("Decompressed table:\n {:#?}", map);
    println!("Uncompressed message length: {}", message_length);

    for _ in 0..message_length {
        for k in map.keys() {
            let ref curr_bits = map[k];
            let peeked = unpacker.peek(curr_bits.len() as i32);
            if peeked.len() == curr_bits.len() && peeked.iter().zip(curr_bits).all(|(a,b)| { a == b}) {
                // println!("{}", *k);
                message.push(*k);
                unpacker.read_bits(curr_bits.len() as i32);
                break;
            }
        }
    }

    message
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_chars(){
        let s = String::from("abbcccc");
        let counts = count_chars(&s);

        assert_eq!(3, counts.keys().len());
        assert_eq!(1, counts[&'a']);
        assert_eq!(2, counts[&'b']);
        assert_eq!(4, counts[&'c']);
    }

    #[test]
    fn test_building_table(){
        let a = HuffmanNode::new_leaf('a', 1);
        let b = HuffmanNode::new_leaf('b', 2);
        let c = HuffmanNode::new_leaf('c', 4);

        let ab = HuffmanNode::new_node(Box::new(a), Box::new(b));
        let root = HuffmanNode::new_node(Box::new(ab), Box::new(c));

        let mut table = HuffmanDictionnary::new();
        table.build_table(&Some(Box::new(root)));

        assert_eq!(3, table.table.keys().len());
        assert_eq!(vec![0, 0], table.table[&'a']);
        assert_eq!(vec![0, 1], table.table[&'b']);
        assert_eq!(vec![1], table.table[&'c']);
    }

    #[test]
    fn test_building_tree (){
        let tree = build_tree("abbcccc");

        assert_eq!(&tree.left.as_ref().unwrap().left.as_ref().unwrap().key, &Some('a'));
        assert_eq!(&tree.left.as_ref().unwrap().right.as_ref().unwrap().key, &Some('b'));
        assert_eq!(&tree.right.as_ref().unwrap().key, &Some('c'));
    }

    #[test]
    fn test_compress_decompress (){
        let abbcccc = String::from("abbcccc");
        let plop = String::from("plop");
        assert_eq!(abbcccc, decompress(compress(&abbcccc)));
        assert_eq!(plop, decompress(compress(&plop)));
    }
}
